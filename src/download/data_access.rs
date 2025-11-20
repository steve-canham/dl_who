use sqlx::{Pool, Postgres};
use std::path::PathBuf;
use chrono::{Utc};
use std::collections::HashMap;
use crate::{err::AppError, DownloadResult};
use crate:: download::file_models::{WHOSummary, SecondaryId};


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
    let sql = r#"Insert into der.who_file_dls(dl_id, file_path, date_dl, 
                num_checked, num_downloaded, num_added) 
                values($1, $2, $3, $4, $5, $6)"#;
    let res = sqlx::query(sql).bind(dl_id).bind(source_path).bind(date_dl)
                .bind(file_res.num_checked).bind(file_res.num_downloaded).bind(file_res.num_added)
                .execute(pool).await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?; 
    Ok(res.rows_affected() == 1)
}


pub async fn add_contents_record(file_path: &PathBuf, source_tots: &mut HashMap<i32, i32>, pool: &Pool<Postgres>) -> Result<u64, AppError> {

    let source_path = file_path.to_str().unwrap().replace("\\\\", "/").replace("\\", "/");   
    let mut source_ids = Vec::<i32>::new();
    let mut tots = Vec::<i32>::new();
    for (k, v) in source_tots.drain() {
        source_ids.push(k);
        tots.push(v)
    }

    // ?? delete existing first ?? 
    // Use the file path to delete previous records relating to this file.
    // let sql ...

    let sql = r#"Insert into der.who_file_contents (file_path, source_id, num_found)
                 select $1, a.*
                    from
                    (select * from UNNEST($2::int[], $3::int[])) as a"#;
               let res = sqlx::query(sql).bind(source_path)
               .bind(source_ids).bind(tots)
               .execute(pool).await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?; 

    Ok(res.rows_affected())
}


pub async fn store_who_summary(rec: WHOSummary, full_path: PathBuf, pool: &Pool<Postgres>) -> Result<bool, AppError> {

    // WHO summary data needs to be modified before storage in db.

    let reg_ids: Option<Vec::<String>>;
    let oth_ids: Option<Vec::<String>>;
    (reg_ids, oth_ids) = split_secids(rec.secondary_ids);
  
    let now = Utc::now();
    let local_path = if full_path == PathBuf::from("") {
        None
    }   
    else {
        Some(full_path.to_str().unwrap().replace("\\\\", "/").replace("\\", "/"))   // to support Windows
    };   
        
    let mut sql = format!("SELECT EXISTS(SELECT 1 from bas.{} where sd_sid = '{}')", rec.table_name, rec.sd_sid); 
    let record_exists = sqlx::query_scalar(&sql).fetch_one(pool).await
                        .map_err(|e| AppError::SqlxError(e, sql.clone()))?;

    if record_exists {
            
        // Update with new details.
        
        sql = "Update bas.".to_string() + &rec.table_name + 
                        r#" SET source_id = $1, remote_url = $2, title = $4, 
                        study_type = $5, study_status = $6, reg_sec_ids = $7, oth_sec_ids = $8, 
                        reg_year = $9, reg_month = $10, reg_day = $11, 
                        enrol_year = $12, enrol_month = $13, enrol_day = $14,
                        results_yes_no = $15, results_url_link = $16, results_url_protocol = $17,  
                        results_date_posted = $18, results_date_first_pub = $19, results_date_completed = $20, 
                        country_list = $21, last_revised_in_who = $22, last_who_dl_id = $23,
                        last_edited_in_sys = $24, local_path = $25
                        where sd_sid = $3"#;
    }
    else {
            
        // Create as a new record.

        sql = "INSERT INTO bas.".to_string() + &rec.table_name + 
                    r#" (source_id, remote_url, sd_sid, title, study_type, study_status, 
                    reg_sec_ids, oth_sec_ids, 
                    reg_year, reg_month, reg_day, enrol_year, enrol_month, enrol_day,
                    results_yes_no, results_url_link, results_url_protocol, 
                    results_date_posted, results_date_first_pub, results_date_completed,
                    country_list, last_revised_in_who, last_who_dl_id, last_edited_in_sys, local_path)
                VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 
                $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
                $21, $22, $23, $24, $25)"#;
    }

    sqlx::query(&sql)
    .bind(rec.source_id).bind(rec.remote_url)
    .bind(rec.sd_sid).bind(rec.title)
    .bind(rec.study_type).bind(rec.study_status)
    .bind(reg_ids).bind(oth_ids)
    .bind(rec.reg_year).bind(rec.reg_month).bind(rec.reg_day)
    .bind(rec.enrol_year).bind(rec.enrol_month).bind(rec.enrol_day)
    .bind(rec.results_yes_no).bind(rec.results_url_link).bind(rec.results_url_protocol)
    .bind(rec.results_date_posted).bind(rec.results_date_first_pub).bind(rec.results_date_completed)
    .bind(rec.country_list).bind(rec.date_last_rev).bind(rec.dl_id).bind(now).bind(local_path)
    .execute(pool).await
    .map_err(|e| AppError::SqlxError(e, sql))?;

    Ok(!record_exists)   // return whether record needed to be added or not

}
 

fn split_secids (ids: Option<Vec<SecondaryId>>) -> (Option<Vec<String>>, Option<Vec<String>>) {
    
    let mut reg_ids = Vec::<String>::new();
    let mut oth_ids = Vec::<String>::new();

    match ids {
        Some(sids) => {
            if sids.len() > 0 {
                for secid in sids {
                   if secid.sec_id_type_id == 11 {
                       reg_ids.push(format!("{}::{}", secid.sec_id_source, secid.processed_id))
                   }
                   else {
                       oth_ids.push(secid.sec_id)
                   }
                }

                let reg_sec_ids = match reg_ids.len() {
                    0 => None,
                  _ => Some(reg_ids)
                };
                let oth_sec_ids = match oth_ids.len() {
                    0 => None,
                    _ => Some(oth_ids)
                };
                (reg_sec_ids, oth_sec_ids)
           }
           else {
            (None, None)
           }
        },
        None => (None, None),
    }
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

*/