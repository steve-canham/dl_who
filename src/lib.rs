pub mod setup;
pub mod err;
mod who;
mod data;

use setup::cli_reader;
use err::AppError;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct DownloadResult {
    pub num_checked: i32,
    pub num_downloaded: i32,
    pub num_added: i32,
}

impl DownloadResult {
    pub fn new() -> Self {
        DownloadResult {  
        num_checked: 0,
        num_downloaded: 0,
        num_added: 0,
        }
   }

   pub fn add(&self, other: DownloadResult ) -> Self {
    DownloadResult {  
        num_checked: self.num_checked + other.num_checked,
        num_downloaded: self.num_downloaded + other.num_downloaded,
        num_added: self.num_added + other.num_added,
    }
}
}

pub async fn download(args: Vec<OsString>) -> Result<(), AppError> {
    
    // If no config file the command line arguments are forced into
    // the equivalent of a user's initialisation request. Otherwise
    // they are read using the CLAP based CLI reader.

    let cli_pars = cli_reader::fetch_valid_arguments(args)?;
    let config_file = PathBuf::from("./app_config.toml");
    let config_string: String = fs::read_to_string(&config_file)
                    .map_err(|e| AppError::IoReadErrorWithPath(e, config_file))?;
    
    let params = setup::get_params(cli_pars, &config_string)?;

    setup::establish_log(&params)?;
    let pool = setup::get_db_pool().await?;  // pool for the monitoring db
    let json_path = params.json_data_path;

    let dl_id = data::get_next_download_id(&pool).await?;
    let mut dl_res = DownloadResult::new();
   
    match params.dl_type {

        501 => {

            // Default - processing of stored WHO csv files not yet processed (was type 113).

            let source_folder = params.csv_data_path;
            let _last_file = params.last_file_imported;

            // first need a routine that can identify the files and return them 
            // as a vector of file names, in the correct order, if any...

            let files_to_process = vec!["a", "b", "c"];  // for now!!

            if files_to_process.len() > 0 {
                for f in files_to_process {
                    let file_path: PathBuf = [&source_folder, &PathBuf:: from(f)].iter().collect();
                    let res = who::process_single_file(&file_path, &json_path, dl_id, &pool).await?;
                    dl_res = dl_res.add(res);
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
                let res = who::process_single_file(&file_path, &json_path, dl_id, &pool).await?;
                dl_res = dl_res.add(res);
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
            dl_res = who::process_single_file(&file_path, &json_path, dl_id, &pool).await?;
   
            // Record that file's download summary - source file, overall numbers, date etc. in the database
            // Record the result for each source in the DB
        },

        _ => {

            // shouldn't do anything except report weird dl type code
             
        }
    }
    
    // update dl event record with res details
    data::update_dl_event_record (dl_id, params.dl_type, dl_res, &pool).await?;

    Ok(())  
}
