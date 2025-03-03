mod data_models;

use sqlx::{Pool, Postgres};
use std::path::PathBuf;
use chrono::{NaiveDate, Utc};

use crate::err::AppError;


pub async fn update_who_study_mon(db_name: &String, sd_sid: &String, remote_url: &Option<String>, dl_id: i32,
                     record_date:&String, full_path: &PathBuf, pool: &Pool<Postgres>) -> Result<bool, AppError> {

        // Get connection string for this database
       
        let added = false; // indicates if a new record or update of an existing one

        let sql = r#"SELECT EXISTS(SELECT 1 from mon."#.to_string() 
                        + db_name + r#" where sd_sid = '$1');"#;
        let mon_record_exists = sqlx::query_scalar(&sql).bind(sd_sid).fetch_one(pool).await
                        .map_err(|e| AppError::SqlxError(e, sql))?;

        if mon_record_exists {
            // row already exists - update with new details
            // do nothing for now...
        }
        else {
            // create as a new record
            let now = Utc::now();
            let last_revised: Option<NaiveDate> = match NaiveDate::parse_from_str(record_date, "%Y-%m-%d") {
                Ok(d) => Some(d),
                Err(_) => None
            };
            let local_path = full_path.to_str().unwrap().to_string().replace("\\\\", "/");
            let sql = r#"Insert into mon."#.to_string() + db_name + r#"(sd_sid, remote_url,	last_revised,
	                    local_path, last_dl_id, last_downloaded) values ($1, $2, $3, $4, $5, $6)"#;
            sqlx::query(&sql).bind(sd_sid).bind(remote_url).bind(last_revised)    
            .bind(local_path).bind(dl_id).bind(now).execute(pool).await
                    .map_err(|e| AppError::SqlxError(e, sql))?;       
        }


/* 
        let sql = r#"select id, sd_sid, remote_url, last_revised,
                    local_path, last_dl_id, last_downloaded, 
                    last_harvest_id, last_harvested, last_import_id, last_imported 
                    from mn.source_data where sd_sid = '$1';"#;

*/
                 
            
 /*
                        public bool UpdateWhoStudyLog(string db_name, string sd_sid, string? remote_url,
        int? dl_id, DateTime? last_revised_date, string? local_path)
    {
        bool added = false; // indicates if a new record or update of an existing one

        // Get the source data record and modify it or add a new one.
        
        StudyFileRecord? file_record = FetchStudyFileRecord(sd_sid, db_name);
        try
        {
            if (file_record is null)
            {
                file_record = new StudyFileRecord(sd_sid, remote_url, dl_id,
                    last_revised_date, local_path);
                InsertStudyFileRec(file_record, db_name);
                added = true;
            }
            else
            {
                file_record.remote_url = remote_url;
                file_record.last_dl_id = dl_id;
                file_record.last_revised = last_revised_date;
                file_record.last_downloaded = DateTime.Now;
                file_record.local_path = local_path;

                UpdateStudyFileRec(file_record, db_name);
            }

            return added;
        }
        catch(Exception e)
        {
            _logging_helper.LogError("In UpdateStudyDownloadLog: " + e.Message);
            return false;
        }
    }
    
                         */
        Ok(added)
}

pub async fn get_next_download_id(pool: &Pool<Postgres>) -> Result<i32, AppError>{

    let sql = "select max(id) from evs.dl_events ";
    let last_id: i32 = sqlx::query_scalar(sql).fetch_one(pool)
                      .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;
    let new_id = last_id + 1;
    
    // Create the new record (to be updated later).

    let sql = "Insert into evs.dl_events(id) values ($1)";
    sqlx::query(sql).bind(new_id).execute(pool)
             .await.map_err(|e| AppError::SqlxError(e, sql.to_string()))?;

    Ok(new_id)
   
}

/* 
pub fn update_dl_record()  {

    /*
    public bool UpdateDLEventRecord(DLEvent dl)
    {
        // Occasionally a simple insert of a new record at the end of a download process can result 
        // in an error - if a long running DL takes place at the same time as a pre-scheduled one.
        // In that case the id originally set up will have already been used.
        // To get round that a new DL record should be created at the beginning of each DL, 
        // and then updated with the dl details at the end.
        
        using NpgsqlConnection Conn = new(monConnString);
        return Conn.Update(dl);
    }
     */
}


pub fn is_who_test_study() -> bool {

    /*
    public bool IsWHOTestStudy(string dbname, string sd_sid)
    {
        string whoConnString = _credentials.GetConnectionString(dbname);
        using NpgsqlConnection conn = new(whoConnString);
        string sql_string = @$"select for_testing
                    from mn.source_data where sd_sid = '{sd_sid}';";
        bool? res = conn.QueryFirstOrDefault<bool?>(sql_string);
        return res == true;
    }

     */
    false
}
*/


/*

public class MonDataLayer : IMonDataLayer
{
    private readonly ILoggingHelper _logging_helper;
    private readonly ICredentials _credentials;
    
    private readonly string monConnString;
    private readonly string pubmedAPIKey;
    private Source? source;
    private string thisDBconnString = "";
    
    public MonDataLayer(ILoggingHelper logging_helper, ICredentials credentials)
    {
        _logging_helper = logging_helper;
        _credentials = credentials;
        
        monConnString = credentials.GetConnectionString("mon");
        pubmedAPIKey = credentials.GetPubMedApiKey();
    }

    public string PubmedAPIKey => pubmedAPIKey;
    
    public Credentials Credentials => (Credentials)_credentials;

    public Source? FetchSourceParameters(int source_id)
    {
        using NpgsqlConnection Conn = new(monConnString);
        source = Conn.Get<Source>(source_id);
        thisDBconnString = _credentials.GetConnectionString(source.database_name!);
        return source;
    }

    public DateTime? ObtainLastDownloadDate(int source_id)
    {
        using NpgsqlConnection Conn = new(monConnString);
        string sql_string = $@"select max(time_ended) from sf.dl_events 
                               where source_id = {source_id}";
        return Conn.QuerySingleOrDefault<DateTime>(sql_string);
    }


    public DateTime? ObtainLastDownloadDateWithFilter(int source_id, int filter_id)
    {
        using NpgsqlConnection Conn = new(monConnString);
        string sql_string = $@"select max(time_ended) from sf.dl_events 
                               where source_id = {source_id} and filter_id = {filter_id}";
        return Conn.QuerySingleOrDefault<DateTime>(sql_string);
    }


    public DLType FetchTypeParameters(int dl_type_id)
    {
        using NpgsqlConnection Conn = new(monConnString);
        return Conn.Get<DLType>(dl_type_id);
    }

   
    public StudyFileRecord? FetchStudyFileRecord(string sd_sid, string db_name = "")
    {
        string connString = db_name == "" ? thisDBconnString 
                                          : _credentials.GetConnectionString(db_name); 

        using NpgsqlConnection conn = new(connString);
        string sql_string = @$"select id, sd_sid, remote_url, last_revised,
                    local_path, last_dl_id, last_downloaded, 
                    last_harvest_id, last_harvested, last_import_id, last_imported 
                    from mn.source_data where sd_sid = '{sd_sid}';";
        return conn.Query<StudyFileRecord>(sql_string).FirstOrDefault();
    }

    
    public ObjectFileRecord? FetchObjectFileRecord(string sd_oid)
    {
        using NpgsqlConnection conn = new(thisDBconnString);
        string sql_string = @$"select id, sd_oid, remote_url, last_revised, 
                   local_path, last_dl_id, last_downloaded, 
                   last_harvest_id, last_harvested, last_import_id, last_imported 
                   from mn.source_data where sd_oid = '{sd_oid}';";
        return conn.Query<ObjectFileRecord>(sql_string).FirstOrDefault();
    }
        

    private void UpdateStudyFileRec(StudyFileRecord file_record, string db_name = "")
    {
        string connString = db_name == "" ? thisDBconnString 
            : _credentials.GetConnectionString(db_name); 
        using NpgsqlConnection conn = new(connString);
        conn.Update(file_record);
    }
   
    private void UpdateObjectFileRec(ObjectFileRecord file_record)
    {
        using NpgsqlConnection conn = new(thisDBconnString);
        conn.Update(file_record);
    }

    private void InsertStudyFileRec(StudyFileRecord file_record, string db_name = "")
    {
        string connString = db_name == "" ? thisDBconnString 
            : _credentials.GetConnectionString(db_name); 
        using NpgsqlConnection conn = new(connString);
        conn.Insert(file_record);
    }
   
    private void InsertObjectFileRec(ObjectFileRecord file_record)
    {
        using NpgsqlConnection conn = new(thisDBconnString);
        conn.Insert(file_record);
    }
    
    public bool IsTestStudy(string sd_sid)
    {
        using NpgsqlConnection conn = new(thisDBconnString);
        string sql_string = @$"select for_testing
                    from mn.source_data where sd_sid = '{sd_sid}';";
        bool? res = conn.QueryFirstOrDefault<bool?>(sql_string);
        return res == true;
    }
    
    
    public bool IsTestObject(string sd_oid)
    {
        string sql_string = @$"select for_testing
                    from mn.source_data where sd_oid = '{sd_oid}';";
        using NpgsqlConnection conn = new(thisDBconnString);
        bool? res = conn.QueryFirstOrDefault<bool?>(sql_string);
        return res == true;
    }


    public bool UpdateWhoStudyLog(string db_name, string sd_sid, string? remote_url,
        int? dl_id, DateTime? last_revised_date, string? local_path)
    {
        bool added = false; // indicates if a new record or update of an existing one

        // Get the source data record and modify it or add a new one.
        
        StudyFileRecord? file_record = FetchStudyFileRecord(sd_sid, db_name);
        try
        {
            if (file_record is null)
            {
                file_record = new StudyFileRecord(sd_sid, remote_url, dl_id,
                    last_revised_date, local_path);
                InsertStudyFileRec(file_record, db_name);
                added = true;
            }
            else
            {
                file_record.remote_url = remote_url;
                file_record.last_dl_id = dl_id;
                file_record.last_revised = last_revised_date;
                file_record.last_downloaded = DateTime.Now;
                file_record.local_path = local_path;

                UpdateStudyFileRec(file_record, db_name);
            }

            return added;
        }
        catch(Exception e)
        {
            _logging_helper.LogError("In UpdateStudyDownloadLog: " + e.Message);
            return false;
        }
    }
    
    
    public bool UpdateStudyLog(string sd_sid, string? remote_url,
                     int? dl_id, DateTime? last_revised_date, string? local_path)
    {
        bool added = false; // indicates if a new record or update of an existing one

        // Get the source data record and modify it or add a new one.
        
        StudyFileRecord? file_record = FetchStudyFileRecord(sd_sid);
        try
        {
            if (file_record is null)
            {
                file_record = new StudyFileRecord(sd_sid, remote_url, dl_id,
                                                last_revised_date, local_path);
                InsertStudyFileRec(file_record);
                added = true;
            }
            else
            {
                file_record.remote_url = remote_url;
                file_record.last_dl_id = dl_id;
                file_record.last_revised = last_revised_date;
                file_record.last_downloaded = DateTime.Now;
                file_record.local_path = local_path;

                UpdateStudyFileRec(file_record);
            }

            return added;
        }
        catch(Exception e)
        {
            _logging_helper.LogError("In UpdateStudyDownloadLog: " + e.Message);
            return false;
        }
    }

    public bool UpdateObjectLog(string sd_oid, string? remote_url,
                     int? dl_id, DateTime? last_revised_date, string? local_path)
    {
        bool added = false; // indicates if a new record or update of an existing one

        // Get the source data record and modify it or add a new one...
        ObjectFileRecord? file_record = FetchObjectFileRecord(sd_oid);

        if (file_record is null)
        {
            file_record = new ObjectFileRecord(sd_oid, remote_url, dl_id,
                                            last_revised_date, local_path);
            InsertObjectFileRec(file_record);
            added = true;
        }
        else
        {
            file_record.remote_url = remote_url;
            file_record.last_dl_id = dl_id;
            file_record.last_revised = last_revised_date;
            file_record.last_downloaded = DateTime.Now;
            file_record.local_path = local_path;
            UpdateObjectFileRec(file_record);
        }

        return added;
    }

    public bool Downloaded_recently(string sd_sid, int days_ago)
    {
        string sql_string = $@"select id from mn.source_data 
                               where last_downloaded::date >= now()::date - {days_ago} 
                               and sd_sid = '{sd_sid}'";
        using NpgsqlConnection conn = new(thisDBconnString);
        return conn.Query<int>(sql_string).FirstOrDefault() > 0;
    }
     
    
    public bool Downloaded_recently_with_link(string details_link, int days_ago)
    {
        string sql_string = $@"select id from mn.source_data 
                               where last_downloaded::date >= now()::date - {days_ago} 
                               and remote_url = '{details_link}'";
        using NpgsqlConnection conn = new(thisDBconnString);
        return conn.Query<int>(sql_string).FirstOrDefault() > 0;
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

        catch (Exception e)
        {
            _logging_helper.LogLine("Error in trying to save file at " +
                                Path.Combine(folder_path, sid + ".json") +
                                   ":: " + e.Message);
            return false;
        }
    }



}



*/