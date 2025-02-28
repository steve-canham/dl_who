pub mod setup;
pub mod err;
mod who;
mod data;

use setup::cli_reader;
use err::AppError;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct DownloadResult {
    pub time_started: DateTime<Utc>,
    pub time_ended: DateTime<Utc>,
    pub num_records_checked: i32,
    pub num_records_downloaded: i32,
    pub num_records_added: i32,
}

impl DownloadResult {
    pub fn new() -> Self {
        DownloadResult {  
        time_started: Utc::now(),
        time_ended: Utc::now(),
        num_records_checked: 0,
        num_records_downloaded: 0,
        num_records_added: 0,
        }
   }

}

pub async fn run(args: Vec<OsString>) -> Result<(), AppError> {
    
    // If no config file the command line arguments are forced into
    // the equivalent of a user's initialisation request. Otherwise
    // they are read using the CLAP based CLI reader.

    let cli_pars = cli_reader::fetch_valid_arguments(args)?;
    let config_file = PathBuf::from("./app_config.toml");
    let config_string: String = fs::read_to_string(&config_file)
                    .map_err(|e| AppError::IoReadErrorWithPath(e, config_file))?;
    
    let params = setup::get_params(cli_pars, &config_string)?;

    setup::establish_log(&params)?;
    let _pool = setup::get_db_pool().await?;
    let json_path = params.json_data_path;

    let mut res = DownloadResult::new();
   
    match params.dl_type {

        501 => {

            // Default - processing of stored WHO csv files not yet processed (was type 113).

            let source_folder = params.csv_data_path;
            let _last_file = params.last_file_imported;

            // first need a routine that can identify the files and return them 
            // as a vector of file names, in the correct order, if any...

            let files_to_process = vec!["a", "b", "c"];

            if files_to_process.len() > 0 {
                for f in files_to_process {
                    let file_path: PathBuf = [&source_folder, &PathBuf:: from(f)].iter().collect();
                    let _res = who::process_single_file(&file_path, &json_path, &mut res)?;

                    // record the event in the dl events table:
                    // Record that file's download - source file, overall numbers, date etc. in the database
                }

                // aggregate figures as we go through the loop, for final feedback
                // but do not DB them ??
            }

        },

        502 => {

            // Processing of a full data download (20+ files) (was type 103).
            
            let source_folder = params.csv_full_path;
            let file_num = params.full_file_num;
            let file_stem = params.full_file_stem;

            for i in 1..file_num {
                let file_name = file_stem.clone() + &(format!("{:0>3}", i));
                let file_path: PathBuf = [&source_folder, &PathBuf:: from(file_name)].iter().collect();
                let _res = who::process_single_file(&file_path, &json_path, &mut res)?;

                // record the event in the dl events table:
                // Record that file's download - source file, overall numbers, date etc. in the database
            }
                       
            // aggregate figures as we go through the loop, for final feedback
            // but do not DB them ??

        },

        503 => {

            // Processing of a single designated file.
            let source_folder = params.csv_data_path;

            let file_name = params.target;
            let file_path: PathBuf = [source_folder, PathBuf:: from(file_name)].iter().collect();
            let _res = who::process_single_file(&file_path, &json_path, &mut res)?;

            // record the event in the dl events table:
            // Record that file's download - source file, overall numbers, date etc. in the database
        },

        _ => {

            // shouldn't do anything except report weird dl type code
             
        }
    }



    Ok(())  
}
