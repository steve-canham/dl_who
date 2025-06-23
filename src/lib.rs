pub mod setup;
pub mod err;
mod download;
mod aggregate;

use setup::cli_reader;
use err::AppError;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use download::data_access::{get_next_download_id, update_dl_event_record};

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
    let mon_pool = setup::get_mon_db_pool().await?;  // pool for the monitoring db
    let src_pool = setup::get_src_db_pool().await?;  // pool for the source specific db

    if params.doing_agg_only {
        
        // do the data aggregation process using the data already in the DB
        // rather than download data from files

        aggregate::aggregate_who_data(&src_pool).await?;

    }
    else {

        let json_path = params.json_data_path;
        let dl_id = get_next_download_id(&mon_pool).await?;
        let mut dl_res = DownloadResult::new();
        
        match params.dl_type {

            501 => {
                // Default - processing of stored WHO csv files not yet processed (was type 113).

                let source_folder = params.csv_data_path;
                let last_file = params.last_file_imported;

                // first need a routine that can identify the files and return them 
                // as a vector of file names, in the correct order, if any...

                let files_to_process = setup::get_files_to_process(&source_folder, &last_file)?;  // **** Ordering to check ****

                if files_to_process.len() > 0 {
                    for f in files_to_process {
                        let file_path: PathBuf = [&source_folder, &PathBuf:: from(f)].iter().collect();
                        let res = download::process_single_file(&file_path, &json_path, dl_id, &mon_pool, &src_pool).await?;
                        dl_res = dl_res.add(res);
                    }
                }
            },

            502 => {
                // Processing of a full data download (20+ files) (was type 103).
                
                let source_folder = params.csv_full_path;
                let file_num = params.full_file_num + 1;
                let file_stem = params.full_file_stem;

                for i in 4..file_num {
                    let file_name = file_stem.clone() + &(format!("{:0>3}", i) + ".csv");
                    let file_path: PathBuf = [&source_folder, &PathBuf:: from(file_name)].iter().collect();
                    let res = download::process_single_file(&file_path, &json_path, dl_id, &mon_pool, &src_pool).await?;
                    dl_res = dl_res.add(res);
                }
            },

            503 => {
                // Processing of a single designated file.

                let source_folder = params.csv_data_path;
                let file_name = params.target;
                let file_path: PathBuf = [source_folder, PathBuf:: from(file_name)].iter().collect();
                dl_res = download::process_single_file(&file_path, &json_path, dl_id, &mon_pool, &src_pool).await?;
            },
            
            _ => {    // shouldn't do anything except report weird dl type code
 
                
                
            }
        }

        // Update dl event record with res details

        update_dl_event_record (dl_id, params.dl_type, dl_res, &mon_pool).await?;
    }

    Ok(())  
}
