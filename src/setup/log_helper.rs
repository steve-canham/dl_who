/***************************************************************************
 * Establishes the log for the programme's operation using log and log4rs, 
 * and includes various helper functions.
 * Once established the log file appears to be accessible to any log
 * statement within the rest of the program (after 'use log:: ...').
 ***************************************************************************/

use chrono::Local;
use std::path::PathBuf;
use crate::err::AppError;
use crate::setup::InitParams;

use log::{info, LevelFilter};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};


pub fn setup_log (data_folder: &PathBuf, dl_type: usize) -> Result<log4rs::Handle, AppError> {
    let log_file_path = get_log_file_path(data_folder, dl_type);
    config_log (&log_file_path)
}

fn get_log_file_path(data_folder: &PathBuf, dl_type: usize) -> PathBuf {
    
    let datetime_string = Local::now().format("%m-%d %H%M%S").to_string();
    let log_file_name = format!("WHO DL ({}) {} ", dl_type, datetime_string);
    [data_folder, &PathBuf::from(&log_file_name)].iter().collect()
    
}

fn config_log (log_file_path: &PathBuf) -> Result<log4rs::Handle, AppError> {
    
    // Initially establish a pattern for each log line.

    let log_pattern = "{d(%d/%m %H:%M:%S)}  {h({l})}  {({M}.{L}):>38.48}:  {m}\n";

    // Define a stderr logger, as one of the 'logging' sinks or 'appender's.

    let stderr = ConsoleAppender::builder().encoder(Box::new(PatternEncoder::new(log_pattern)))
        .target(Target::Stderr).build();

    // Define a second logging sink or 'appender' - to a log file (provided path will place it in the current data folder).

    let logfile = FileAppender::builder().encoder(Box::new(PatternEncoder::new(log_pattern)))
            .build(log_file_path)
            .map_err(|e| AppError::IoWriteErrorWithPath(e, log_file_path.to_owned()))?;
    
    // Configure and build log4rs instance, using the two appenders described above

    let config = Config::builder()
        .appender(Appender::builder()
                .build("logfile", Box::new(logfile)),)
        .appender(Appender::builder()
                .build("stderr", Box::new(stderr)),)
        .build(Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Info),
        )
        .map_err(|e| AppError::LogSetupError("Error when creating log4rs configuration".to_string(), e.to_string()))?;

    log4rs::init_config(config)
        .map_err(|e| AppError::LogSetupError("Error when creating log4rs handle".to_string(), e.to_string()))

}


pub fn log_startup_params (ip : &InitParams) {
    
    // Called at the end of set up to record the input parameters
    
    info!("PROGRAM START");
    info!("");
    info!("************************************");
    info!("");
    info!("download type: {}", ip.dl_type.to_string());
    info!("full DL file stem: {}", ip.full_file_stem);
    info!("full DL file num: {}", ip.full_file_num.to_string());
    info!("(previous) last file imported: {}", ip.last_file_imported);
    info!("target file: {:?}", ip.target);
    info!("csv data path: {:?}", ip.csv_data_path);
    info!("csv full data path: {:?}", ip.csv_full_path);
    info!("json data parth: {:?}", ip.json_data_path);
    info!("log folder path: {:?}", ip.log_folder_path);

    info!("");
    info!("************************************");
    info!("");
}

pub fn write_config (config_string: &String) {
    info!("Config file created or modified");
    info!("New file is:");
    info!("{}", config_string);
    info!("");
}