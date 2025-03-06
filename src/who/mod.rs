mod file_models;
mod processor;
pub mod data_access;
pub mod who_helper;
pub mod gen_helper;

use std::collections::HashMap;
use std::path::PathBuf;
use crate::{AppError, DownloadResult};
use data_access::{update_who_study_mon, add_new_single_file_record, add_file_contents_record, store_who_summary};
use file_models::WHOLine;
use std::fs;
use std::io::BufReader;
use std::fs::File;
use csv::ReaderBuilder;
use std::io::Write;
use serde_json::to_string_pretty;
use sqlx::{Pool, Postgres};
use log::info;


pub async fn process_single_file(file_path: &PathBuf, json_path: &PathBuf, dl_id: i32, pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    // Set up source file, csv reader, counters, hash table.

    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let mut csv_rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(buf_reader);
    let mut file_res = DownloadResult::new();
    let mut source_tots: HashMap<i32, i32> = HashMap::new();

     for result in csv_rdr.deserialize() {

        file_res.num_checked +=1;
        if file_res.num_checked % 100 == 0 {
            info!("{} records checked", file_res.num_checked);
        }

        let who_line: WHOLine = match result {
             Ok(w) => w,
             Err(e) => return Err(AppError::CsvError(e, file_res.num_checked.to_string())),
        };
        
        let (source_id, who_rec) = processor::process_line(who_line, file_res.num_checked);

        // Get source and adjust running source totals (even if file not processed further)

        source_tots.entry(source_id).and_modify(|n| *n += 1).or_insert(1);

        if who_rec.is_none() { 
            continue;
        }     
        
        // But if on the happy path - Work out file destination folder and full path.

        let rec = who_rec.unwrap(); 
        let db_name = &rec.db_name;
        let file_folder: PathBuf = [json_path, &PathBuf::from(db_name)].iter().collect();
        if !folder_exists(&file_folder) {
            fs::create_dir_all(&file_folder)?;
        }

        let sd_sid = &rec.sd_sid;
        let file_name = format!("{}.json", sd_sid);
        let full_path: PathBuf = [file_folder, PathBuf::from(&file_name)].iter().collect();

        // Write the JSON string to a file - see if it is a new download, or an existing one.
        // Update database and res accordingly.

        let json_string = to_string_pretty(&rec).unwrap();
        let mut file = File::create(&full_path)?;
        file.write_all(json_string.as_bytes())?;
        
        let added = update_who_study_mon(&db_name, &sd_sid, &rec.remote_url, dl_id,
                        &rec.record_date, &full_path, pool).await?;

        file_res.num_downloaded +=1;
        if added {
            file_res.num_added +=1;
        } 

    }

    // Update database with single file details and 
    // return the aggregate figures in the res struct ... 

    add_new_single_file_record(dl_id, file_path, &file_res, pool).await?;
    add_file_contents_record(dl_id, file_path, &mut source_tots, pool).await?;
    Ok(file_res)

}


pub async fn store_single_file(file_path: &PathBuf, pool: &Pool<Postgres>) -> Result<(), AppError> {

    // Set up source file, csv reader, counters, hash table.

    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let mut csv_rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(buf_reader);

    let mut i = 0;
    for result in csv_rdr.deserialize() {

        i +=1;
        if i % 100 == 0 {
            info!("{} records checked", i);
        }

        let who_line: WHOLine = match result {
             Ok(w) => w,
             Err(e) => return Err(AppError::CsvError(e, i.to_string())),
        };
        
        let rec = match processor::summarise_line(who_line, i)
        {
            Some(w) => w,
            None =>  { continue;},
        };
        
        store_who_summary(rec, pool).await?;             // add or update database record

    }
   
    Ok(())

}



fn folder_exists(folder_name: &PathBuf) -> bool {
    let res = match folder_name.try_exists() {
        Ok(true) => true,
        Ok(false) => false, 
        Err(_e) => false,           
    };
    res
}
