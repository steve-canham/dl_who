mod file_models;
mod processor;
pub mod data_access;
pub mod who_helper;
pub mod gen_helper;

use std::collections::HashMap;
use std::path::PathBuf;
use crate::{AppError, DownloadResult};
use data_access::{add_new_single_file_record, 
    add_contents_record, store_who_summary};
use file_models::WHOLine;
use who_helper::get_db_name;
use std::fs;
use std::io::BufReader;
use std::fs::File;
use csv::ReaderBuilder;
use std::io::Write;
use serde_json::to_string_pretty;
use sqlx::{Pool, Postgres};
use log::info;


pub async fn process_single_file(file_path: &PathBuf, json_path: &PathBuf, dl_id: i32, 
                src_pool: &Pool<Postgres>) -> Result<DownloadResult, AppError> {

    // Set up source file, csv reader, counters, hash table.

    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let mut csv_rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(buf_reader);
    let mut file_res = DownloadResult::new();
    let mut source_tots: HashMap<i32, i32> = HashMap::new();
    info!("");
    info!("Processing file {:?}", file_path);

    for result in csv_rdr.deserialize() {

        file_res.num_checked +=1;
        if file_res.num_checked % 1000 == 0 {
            info!("{} records checked", file_res.num_checked);
        }

        // Obtain the full record from deserialisation

        let who_line: WHOLine = match result {
             Ok(w) => w,
             Err(e) => return Err(AppError::CsvError(e, file_res.num_checked.to_string())),
        };
               
        // Construct the summary record
        
        let rec_summ = match processor::summarise_line(&who_line, dl_id, file_res.num_checked)
        {
            Some(r) => r,
            None => continue,   // some sort of problem occured - should have been logged
        };

        let mut full_path = PathBuf::from("");     
        if rec_summ.source_id != 100120  && rec_summ.source_id != 100126 {           // file production not necessary for these sources
            
            // Process the whole line to get a full WHO record for storage.

            match processor::process_line(who_line, &rec_summ)
            {
                Some (rec) => {

                    let db_name = get_db_name(rec_summ.source_id);
                    let file_folder: PathBuf = [json_path, &PathBuf::from(&db_name)].iter().collect();
                    if !folder_exists(&file_folder) {
                        fs::create_dir_all(&file_folder)?;
                    }

                    let file_name = format!("{}.json", &rec.sd_sid);
                    full_path = [file_folder, PathBuf::from(&file_name)].iter().collect();
                    
                    // Write the JSON string to a file.
                    
                    let json_string = to_string_pretty(&rec).unwrap();
                    let mut file = File::create(&full_path)?;
                    file.write_all(json_string.as_bytes())?;
                },

                None => continue,  // some sort of problem occured - should have been logged
            }
        }

        // Adjust running source totals.

        let source_id = rec_summ.source_id;
        source_tots.entry(source_id).and_modify(|n| *n += 1).or_insert(1);

        // Store the WHO summary record in the database (whether a file was produced or not).

        let added = store_who_summary(rec_summ, full_path, src_pool).await?;           

        // Update the Download summary struct.

        file_res.num_downloaded +=1;
        if added {
            file_res.num_added +=1;
        } 
    }

    info!("{} records checked in total for this file", file_res.num_checked);
    info!("---------------------------------------------------");

    // Update database with single file details and 
    // return the aggregate figures in the res struct ... 

    add_new_single_file_record(dl_id, file_path, &file_res, src_pool).await?;
    add_contents_record(file_path, &mut source_tots, src_pool).await?;

    Ok(file_res)

}


fn folder_exists(folder_name: &PathBuf) -> bool {
    let res = match folder_name.try_exists() {
        Ok(true) => true,
        Ok(false) => false, 
        Err(_e) => false,           
    };
    res
}
