use sqlx::{Pool, Postgres};
use std::path::PathBuf;
use chrono::{NaiveDate, Utc};
use std::collections::HashMap;
use crate::{err::AppError, DownloadResult};
use crate:: who::file_models::WHOSummary;

pub async fn update_who_study_mon(db_name: &String, sd_sid: &String, remote_url: &Option<String>, dl_id: i32,
                     record_date:&String, full_path: &PathBuf, pool: &Pool<Postgres>) -> Result<bool, AppError> {

        let mut added = false;          // indicates if will be a new record or update of an existing one
        let now = Utc::now();
        let last_revised: Option<NaiveDate> = match NaiveDate::parse_from_str(record_date, "%Y-%m-%d") {
            Ok(d) => Some(d),
            Err(_) => None
        };
        let local_path = full_path.to_str().unwrap().replace("\\\\", "/").replace("\\", "/");     // assumes utf-8 characters
        let sql = format!("SELECT EXISTS(SELECT 1 from mon.{} where sd_sid = '{}')", db_name, sd_sid); 
        let mon_record_exists = sqlx::query_scalar(&sql).fetch_one(pool).await
                        .map_err(|e| AppError::SqlxError(e, sql))?;
        if mon_record_exists {
            
            // Row already exists - update with new details.
           
            let sql = "Update mon.".to_string() + db_name + r#" set 
                        remote_url = $1,
                        last_revised = $2,
                        local_path = $3,
                        last_dl_id = $4,
                        last_downloaded = $5
                        where sd_sid = $6;"#;
            sqlx::query(&sql).bind(remote_url).bind(last_revised).bind(local_path) 
                    .bind(dl_id).bind(now).bind(sd_sid).execute(pool).await
                    .map_err(|e| AppError::SqlxError(e, sql))?;       
        }
        else {
            
            // Create as a new record.
            
            let sql = "Insert into mon.".to_string() + db_name + r#"(sd_sid, remote_url, last_revised,
	                    local_path, last_dl_id, last_downloaded) values ($1, $2, $3, $4, $5, $6)"#;
            sqlx::query(&sql).bind(sd_sid).bind(remote_url).bind(last_revised)    
            .bind(local_path).bind(dl_id).bind(now).execute(pool).await
                    .map_err(|e| AppError::SqlxError(e, sql))?;     
            added = true;  
        }

        Ok(added)
}

pub async fn get_next_download_id(pool: &Pool<Postgres>) -> Result<i32, AppError>{

    let sql = "select max(id) from evs.dl_events ";
    let last_id: i32 = sqlx::query_scalar(sql).fetch_one(pool)
                      .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    let new_id = last_id + 1;
    
    // Create the new record (to be updated later).
    let now = Utc::now();
    let sql = "Insert into evs.dl_events(id, source_id, time_started) values ($1, $2, $3)";
    sqlx::query(sql).bind(new_id).bind(100115).bind(now).execute(pool)
             .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(new_id)
}


pub async fn update_dl_event_record (dl_id: i32, type_id: i32, dl_res: DownloadResult, pool: &Pool<Postgres>) ->  Result<bool, AppError> {
     
    let now = Utc::now();
    let sql = r#"Update evs.dl_events set 
             num_records_checked = $1,
             num_records_downloaded = $2,
             num_records_added = $3,
             time_ended = $4,
             type_id = $5
             where id = $6"#;
    let res = sqlx::query(sql).bind(dl_res.num_checked).bind(dl_res.num_downloaded).bind(dl_res.num_added)
          .bind(now).bind(type_id).bind(dl_id).execute(pool)
             .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?; 
    Ok(res.rows_affected() == 1)
}


pub async fn add_new_single_file_record(dl_id: i32, file_path: &PathBuf, file_res: &DownloadResult, pool: &Pool<Postgres>) -> Result<bool, AppError> {

    let source_path = file_path.to_str().unwrap().replace("\\\\", "/").replace("\\", "/");     // assumes utf-8 characters
    let date_dl = Utc::now().date_naive();
    let sql = r#"Insert into evs.who_file_dls(dl_id, file_path, date_dl, 
                num_checked, num_downloaded, num_added) 
                values($1, $2, $3, $4, $5, $6)"#;
    let res = sqlx::query(sql).bind(dl_id).bind(source_path).bind(date_dl)
                .bind(file_res.num_checked).bind(file_res.num_downloaded).bind(file_res.num_added)
                .execute(pool).await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?; 
    Ok(res.rows_affected() == 1)
}


pub async fn add_file_contents_record(dl_id: i32, file_path: &PathBuf, source_tots: &mut HashMap<i32, i32>, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let source_path = file_path.to_str().unwrap().replace("\\\\", "/").replace("\\", "/");   
    let mut source_ids = Vec::<i32>::new();
    let mut tots = Vec::<i32>::new();
    for (k, v) in source_tots.drain() {
        source_ids.push(k);
        tots.push(v)
    }
    let sql = r#"Insert into evs.who_file_contents (dl_id, file_path, source_id, num_found)
                 select $1, $2, a.*
                    from
                    (select * from UNNEST($3::int[], $4::int[])) as a"#;
               let res = sqlx::query(sql).bind(dl_id).bind(source_path)
               .bind(source_ids).bind(tots)
               .execute(pool).await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?; 

    Ok(res.rows_affected())
}

pub async fn store_who_summary(rec: WHOSummary, pool: &Pool<Postgres>) -> Result<bool, AppError> {

    let now = Utc::now();
    let sql_prefix = "INSERT INTO sd.".to_string() + &rec.table_name;
    let sql = sql_prefix + r#" (source_id, sd_sid, title, remote_url, study_type, 
                    reg_year, reg_month, reg_day, enrol_year, enrol_month, enrol_day,
                    study_status, results_yes_no, results_url_link, results_url_protocol, 
                    results_date_posted, results_date_first_pub, results_date_completed,
                    country_list, date_last_rev, date_last_edited)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 
                $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)
            ON CONFLICT (sd_sid)
            DO UPDATE SET
                source_id = $1, 
                title = $3, 
                remote_url = $4, 
                study_type = $5, 
                reg_year = $6, 
                reg_month = $7, 
                reg_day = $8, 
                enrol_year = $9, 
                enrol_month = $10, 
                enrol_day = $11,
                study_status = $12,  
                results_yes_no = $13,  
                results_url_link = $14,  
                results_url_protocol = $15,  
                results_date_posted = $16,  
                results_date_first_pub = $17,  
                results_date_completed = $18, 
                country_list = $19,  
                date_last_rev = $20,  
                date_last_edited = $21"#;

    let res = sqlx::query(&sql)
        .bind(rec.source_id).bind(rec.sd_sid).bind(rec.title).bind(rec.remote_url)
        .bind(rec.study_type).bind(rec.reg_year).bind(rec.reg_month).bind(rec.reg_day)
        .bind(rec.enrol_year).bind(rec.enrol_month).bind(rec.enrol_day).bind(rec.study_status)
        .bind(rec.results_yes_no).bind(rec.results_url_link).bind(rec.results_url_protocol)
        .bind(rec.results_date_posted).bind(rec.results_date_first_pub).bind(rec.results_date_completed)
        .bind(rec.country_list).bind(rec.date_last_rev).bind(now)
        .execute(pool).await
        .map_err(|e| AppError::SqlxError(e, sql))?;

    Ok(res.rows_affected() == 1)

} 


/*

pub fn is_who_test_study() -> bool {

    public bool IsWHOTestStudy(string dbname, string sd_sid)
    {
        string whoConnString = _credentials.GetConnectionString(dbname);
        using NpgsqlConnection conn = new(whoConnString);
        string sql_string = @$"select for_testing
                    from mn.source_data where sd_sid = '{sd_sid}';";
        bool? res = conn.QueryFirstOrDefault<bool?>(sql_string);
        return res == true;
    }
    false
}

public bool WriteFile(string sid, string jsonString, string folder_path)
{
    try
    {
        // Write out study record as json.

        string full_path = Path.Combine(folder_path, sid + ".json");
        File.WriteAllText(full_path, jsonString);

        if (IsTestStudy(sid))
        {
            // write out copy of the file in the test folder

            string test_path = _logging_helper.TestFilePath;
            string full_test_path = Path.Combine(test_path, sid + ".json");
            File.WriteAllText(full_test_path, jsonString);
        }
        return true;
    }

}


*/