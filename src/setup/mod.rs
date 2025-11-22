/**********************************************************************************
The setup module, and the get_params function in this file in particular, 
orchestrates the collection and fusion of parameters as provided in 
1) a config toml file, and 
2) command line arguments. 
Where a parameter may be given in either the config file or command line, the 
command line version always over-writes anything from the file.
The module also checks the parameters for completeness (those required will vary, 
depending on the activity specified). If possible, defaults are used to stand in for 
mising parameters. If not possible the program stops with a message explaining the 
problem.
The module also provides a database connection pool on demand.
***********************************************************************************/

pub mod cli_reader;
pub mod config_reader;
pub mod log_helper;

use std::fs;
use std::sync::OnceLock;
use crate::err::AppError;
use sqlx::postgres::{PgPoolOptions, PgConnectOptions, PgPool};
use std::path::PathBuf;
use std::time::Duration;
use sqlx::ConnectOptions;
use config_reader::Config;
use cli_reader::CliPars;
use std::ffi::OsStr;

pub struct InitParams {
    pub dl_type: i32,
    pub full_file_stem: String,
    pub full_file_num: i32,
    pub last_file_imported: String,
    pub target: String,
    pub csv_data_path: PathBuf,
    pub csv_full_path: PathBuf,
    pub json_data_path: PathBuf,
    pub log_folder_path: PathBuf,
    pub doing_agg_only: bool,
}

pub static LOG_RUNNING: OnceLock<bool> = OnceLock::new();

pub fn get_params(cli_pars: CliPars, config_string: &String) -> Result<InitParams, AppError> {

    let config_file: Config = config_reader::populate_config_vars(&config_string)?; 
    
    let folder_pars = config_file.folders;  // guaranteed to exist
    let data_pars = config_file.data_details; 
    
    let empty_pb = PathBuf::from("");
    let empty_str = "".to_string();
    
    if cli_pars.doing_agg_only {

        // File related parameters become irrelevant (apart from the log file)

        Ok(InitParams {
            dl_type: 0,
            full_file_stem: "".to_string(),
            full_file_num: 0,
            last_file_imported: "".to_string(),
            target: "".to_string(),
            csv_data_path: empty_pb.clone(),
            csv_full_path: empty_pb.clone(),
            json_data_path: empty_pb.clone(),
            log_folder_path: folder_pars.log_folder_path,
            doing_agg_only: true,

        })

    }
    else {

        let dl_type = cli_pars.dl_type;
        let mut target = cli_pars.target_file;

        let full_file_stem = data_pars.full_file_stem;
        let full_file_num = data_pars.full_file_num;
        let last_file_imported = data_pars.last_file_imported;
        let csv_data_path = folder_pars.csv_data_path;
        let csv_full_path = folder_pars.csv_full_path;

        let json_data_path = folder_pars.json_data_path;  // already checked as present
        if !folder_exists(&json_data_path) {
            fs::create_dir_all(&json_data_path)?;
        }

        let log_folder_path = folder_pars.log_folder_path;  // already checked as present
        if !folder_exists(&log_folder_path) {
            fs::create_dir_all(&log_folder_path)?;
        }
        
        if target == empty_str {   // from CLI in the first instance
                target = data_pars.target_file;  // otherwise use the config file
        }

        if dl_type == 501 {
            
            // DEFAULT download for WHO data
            // To process any files in the data folder not yet processed, in the correct order

            // Needs csv_data_path to exist (<> "")  

            if csv_data_path == empty_pb  { 
                return Result::Err(AppError::MissingProgramParameter("csv_data_path".to_string()));
            }
            if !folder_exists(&csv_data_path) {
                return Result::Err(AppError::FileSystemError(
                        "Unable to find designated csv source data folder".to_string(), 
                        format!("Path provided was {:?}", csv_data_path)));
            }

            // Needs last_file_imported to exist (<> "")

            if last_file_imported == empty_str  { 
                return Result::Err(AppError::MissingProgramParameter("csv_data_path".to_string()));
            }
            
            // The processing needs to get the files from examining the folder - does not need explicit targets at this stage
            // to identify files not yet downloaded and dated after the last file downloaded date, ordered by date
            // errors on accessing the individual file paths will need to be dealt with there...

        }

        if dl_type == 502 {
            
            // Use full data download 
            // To process files in the full data folder in the correct order

            // need csv_full_path to exist (<> "")  
            
            if csv_full_path == empty_pb  { 
                return Result::Err(AppError::MissingProgramParameter("csv_full_path".to_string()));
            }
            if !folder_exists(&csv_full_path) {
                return Result::Err(AppError::FileSystemError(
                        "Unable to find designated full download data folder".to_string(), 
                        format!("Path provided was {:?}", csv_full_path)));
            }

            // need full_file_stem to exist (but this has a default)
            // need full_file_num to exist  (is > 0)

            if full_file_num == 0  { 
                return Result::Err(AppError::MissingProgramParameter("full_file_num".to_string()));
            }


            // The processing could get the files from a loop - does not need explicit targets at this stage
            // errors on accessing the individual file paths will need to be dealt with there...

        }


        if dl_type == 503 {
            
            // Single file download 
            // To process the designated target file only

            // Needs csv_data_path to exist (<> "")  

            if csv_data_path == empty_pb  { 
                return Result::Err(AppError::MissingProgramParameter("csv_data_path".to_string()));
            }
            if !folder_exists(&csv_data_path) {
                return Result::Err(AppError::FileSystemError(
                        "Unable to find designated csv source data folder".to_string(), 
                        format!("Path provided was {:?}", csv_data_path)));
            }

            // Needs a target file from either the CLI or the config file

            if target == empty_str {   // still
                return Result::Err(AppError::MissingProgramParameter("target file".to_string()));
            }
            
        }

        Ok(InitParams {
            dl_type,
            full_file_stem,
            full_file_num,
            last_file_imported: last_file_imported,
            target: target,
            csv_data_path: csv_data_path,
            csv_full_path: csv_full_path,
            json_data_path: json_data_path,
            log_folder_path: log_folder_path,
            doing_agg_only: cli_pars.doing_agg_only,

        })
    }
}

fn folder_exists(folder_name: &PathBuf) -> bool {
    let res = match folder_name.try_exists() {
        Ok(true) => true,
        Ok(false) => false, 
        Err(_e) => false,           
    };
    res
}


pub async fn get_mon_db_pool() -> Result<PgPool, AppError> {  

    // Establish DB name and thence the connection string
    // (done as two separate steps to allow for future development).
    // Use the string to set up a connection options object and change 
    // the time threshold for warnings. Set up a DB pool option and 
    // connect using the connection options object.

    let db_name = match config_reader::fetch_mon_db_name() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    let db_conn_string = config_reader::fetch_db_conn_string(&db_name)?;  
   
    let mut opts: PgConnectOptions = db_conn_string.parse()
                    .map_err(|e| AppError::DBPoolError("Problem with parsing conection string".to_string(), e))?;
    opts = opts.log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(3));

    PgPoolOptions::new()
        .max_connections(5) 
        .connect_with(opts).await
        .map_err(|e| AppError::DBPoolError(format!("Problem with connecting to database {} and obtaining Pool", db_name), e))
}


pub async fn get_src_db_pool() -> Result<PgPool, AppError> {  

    // Establish DB name and thence the connection string
    // (done as two separate steps to allow for future development).
    // Use the string to set up a connection options object and change 
    // the time threshold for warnings. Set up a DB pool option and 
    // connect using the connection options object.

    let db_name = match config_reader::fetch_src_db_name() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };

    let db_conn_string = config_reader::fetch_db_conn_string(&db_name)?;  
   
    let mut opts: PgConnectOptions = db_conn_string.parse()
                    .map_err(|e| AppError::DBPoolError("Problem with parsing conection string".to_string(), e))?;
    opts = opts.log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(3));

    PgPoolOptions::new()
        .max_connections(5) 
        .connect_with(opts).await
        .map_err(|e| AppError::DBPoolError(format!("Problem with connecting to database {} and obtaining Pool", db_name), e))
}



pub fn establish_log(params: &InitParams) -> Result<(), AppError> {

    if !log_set_up() {  // can be called more than once in context of integration tests
        log_helper::setup_log(&params.log_folder_path, params.dl_type)?;
        LOG_RUNNING.set(true).unwrap(); // should always work
        log_helper::log_startup_params(&params);
    }
    Ok(())
}

pub fn log_set_up() -> bool {
    match LOG_RUNNING.get() {
        Some(_) => true,
        None => false,
    }
}

pub fn get_files_to_process(data_folder: &PathBuf, last_file: &String) -> Result<Vec<String>, AppError> {
    
    let last_file_as_buf = PathBuf::from(last_file);
    let last_file_as_osstr: &OsStr = last_file_as_buf.as_os_str();

    // Get list of csv files in the source folder.
    // 1) Filter out all those directory entries which couldn't be read.
    // 2) Map the directory entries to paths
    // 3) Filter out all paths with extensions other than `csv`

    let csv_paths = std::fs::read_dir(data_folder)?    // Read_dir provides an list of Result<DirEntry, Error>
             .filter_map(|res| res.ok())                   // The list now just the valid DirEntries (.ok generates an option
                                                           // ) but filter_map includes only those with a Some() value
             .map(|dir_entry| dir_entry.path())            // Mapped across to the paths included in the DirEntries
             .filter_map(|path| {
                if path.extension().map_or(false, |ext| ext == "csv") {     //  filter_map filters on Some()) values as 
                                                                            //  generated by the closure. In the closure, the map_or function
                                                                            //  generates true if the path has a .csv extension,
                                                                            //  false otherwise, which determines if the if branch 
                                                                            //  is followed (for Some(path)) or the else.       
                    Some(path)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();      
 
    // Generates a collection of PathBufs.
    // Iterate over and compare with the last_file.
    
    let files = csv_paths.iter()
            .filter_map(|p| p.file_name())
            .filter_map(|f| 
                     if f >  last_file_as_osstr {
                        Some(f)
                     }
                    else {
                        None
                    })
            .map(|f| f.to_str().unwrap().to_string())   // assumes utf-8 characters
            .collect::<Vec<_>>();
   
    Ok(files)
}


// Tests
#[cfg(test)]

mod tests {

    use super::*;
    use std::ffi::OsString;
    #[test]
    fn check_results_with_no_params() {
        let config = r#"
[data]
full_file_stem = "ICTRPFullExport "
full_file_num = "22"
last_file_imported = "20250106 ICTRP.csv"
target_file = "dummy test ICTRP.csv"

[folders]
csv_data_path="/home/steve/Data/MDR source data/WHO"
csv_full_path="/home/steve/Data/MDR source data/WHO/Full export 2025-02"
json_data_path="/home/steve/Data/MDR json files/who"
log_folder_path="/home/steve/Data/MDR_Logs/who"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="who"

        "#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let res = get_params(cli_pars, &config_string).unwrap();
        
        assert_eq!(res.dl_type, 501);
        assert_eq!(res.doing_agg_only, false);

        assert_eq!(res.csv_data_path, PathBuf::from("/home/steve/Data/MDR source data/WHO"));
        assert_eq!(res.csv_full_path, PathBuf::from("/home/steve/Data/MDR source data/WHO/Full export 2025-02"));
        assert_eq!(res.json_data_path, PathBuf::from("/home/steve/Data/MDR json files/who"));
        assert_eq!(res.log_folder_path, PathBuf::from("/home/steve/Data/MDR_Logs/who"));
        
        assert_eq!(res.full_file_stem, "ICTRPFullExport ");
        assert_eq!(res.full_file_num, 22);
        assert_eq!(res.last_file_imported, "20250106 ICTRP.csv");
        assert_eq!(res.target, "dummy test ICTRP.csv");
    }

    #[test]
    fn check_cli_vars_overwrite_config_values() {
        let config = r#"
[data]
full_file_stem = "ICTRPFullExport "
full_file_num = "22"
last_file_imported = "20250106 ICTRP.csv"
target_file = "20250210 ICTRP.csv"

[folders]
csv_data_path="/home/steve/Data/MDR source data/WHO"
csv_full_path="/home/steve/Data/MDR source data/WHO/Full Export 2025-02"
json_data_path="/home/steve/Data/MDR json files/who"
log_folder_path="/home/steve/Data/MDR_Logs/who"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="who"

        "#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target", "-t", "503", "-f", "dummy who file.csv"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let res = get_params(cli_pars, &config_string).unwrap();

        assert_eq!(res.dl_type, 503);
        assert_eq!(res.csv_data_path, PathBuf::from("/home/steve/Data/MDR source data/WHO"));
        assert_eq!(res.csv_full_path, PathBuf::from("/home/steve/Data/MDR source data/WHO/Full Export 2025-02"));
        assert_eq!(res.json_data_path, PathBuf::from("/home/steve/Data/MDR json files/who"));
        assert_eq!(res.log_folder_path, PathBuf::from("/home/steve/Data/MDR_Logs/who"));

        assert_eq!(res.full_file_stem, "ICTRPFullExport ");
        assert_eq!(res.full_file_num, 22);
        assert_eq!(res.last_file_imported, "20250106 ICTRP.csv");
        assert_eq!(res.target, "dummy who file.csv");
    }

    #[test]
    fn check_501_with_min_config() {

        let config = r#"
[data]
last_file_imported = "20250106 ICTRP.csv"

[folders]
csv_data_path="/home/steve/Data/MDR source data/WHO"
json_data_path="/home/steve/Data/MDR json files/who"
log_folder_path="/home/steve/Data/MDR_Logs/who"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="who"
"#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target", "-t", "501"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let res = get_params(cli_pars, &config_string).unwrap();

        assert_eq!(res.dl_type, 501);
        assert_eq!(res.csv_data_path, PathBuf::from("/home/steve/Data/MDR source data/WHO"));
        assert_eq!(res.json_data_path, PathBuf::from("/home/steve/Data/MDR json files/who"));
        assert_eq!(res.log_folder_path, PathBuf::from("/home/steve/Data/MDR_Logs/who"));
        assert_eq!(res.last_file_imported, "20250106 ICTRP.csv");
    }

    #[test]
    #[should_panic]
    fn check_501_no_csv_folder_panics() {

        let config = r#"
[data]
last_file_imported = "20250106 ICTRP.csv"

[folders]
json_data_path="/home/steve/Data/MDR json files/who"
log_folder_path="/home/steve/Data/MDR_Logs/who"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="who"
            "#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target", "-t", "501"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let _res = get_params(cli_pars, &config_string).unwrap();
    }

    #[test]
    #[should_panic]
    fn check_501_no_last_file_panics() {

        let config = r#"
[data]

[folders]
csv_data_path="/home/steve/Data/MDR source data/WHO"
json_data_path="/home/steve/Data/MDR json files/who"
log_folder_path="/home/steve/Data/MDR_Logs/who"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="who"
"#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target", "-t", "501"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let _res = get_params(cli_pars, &config_string).unwrap();
    }

    #[test]
    fn check_502_with_min_config() {

        let config = r#"
[data]
full_file_num = "22"

[folders]
csv_full_path="/home/steve/Data/MDR source data/WHO/Full Export 2025-02"
json_data_path="/home/steve/Data/MDR json files/who"
log_folder_path="/home/steve/Data/MDR logs/who"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="who"
"#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target", "-t", "502"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let res = get_params(cli_pars, &config_string).unwrap();

        assert_eq!(res.dl_type, 502);
        assert_eq!(res.csv_full_path, PathBuf::from("/home/steve/Data/MDR source data/WHO/Full Export 2025-02"));
        assert_eq!(res.json_data_path, PathBuf::from("/home/steve/Data/MDR json files/who"));
        assert_eq!(res.log_folder_path, PathBuf::from("/home/steve/Data/MDR logs/who"));
        assert_eq!(res.full_file_stem, "ICTRPFullExport ");
        assert_eq!(res.full_file_num, 22);
    }

    #[test]
    #[should_panic]
    fn check_502_no_full_path_panics() {

        let config = r#"
[data]
full_file_num = "22"

[folders]
json_data_path="/home/steve/Data/MDR json files/who"
log_folder_path="/home/steve/Data/MDR_Logs/who"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="who"
            "#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target", "-t", "502"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let _res = get_params(cli_pars, &config_string).unwrap();
    }

    #[test]
    #[should_panic]
    fn check_502_no_full_file_num_panics() {

        let config = r#"
[data]

[folders]
json_data_path="/home/steve/Data/MDR json files/who"
log_folder_path="/home/steve/Data/MDR_Logs/who"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="who"
            "#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target", "-t", "502"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let _res = get_params(cli_pars, &config_string).unwrap();
    }

    #[test]
    fn check_503_with_min_config() {

        let config = r#"
[data]
target_file = "dummy test ICTRP.csv"

[folders]
csv_data_path="/home/steve/Data/MDR source data/WHO"
json_data_path="/home/steve/Data/MDR json files/who"
log_folder_path="/home/steve/Data/MDR_Logs/who"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="who"
            "#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target", "-t", "503"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let res = get_params(cli_pars, &config_string).unwrap();

        assert_eq!(res.dl_type, 503);
        assert_eq!(res.csv_data_path, PathBuf::from("/home/steve/Data/MDR source data/WHO"));
        assert_eq!(res.json_data_path, PathBuf::from("/home/steve/Data/MDR json files/who"));
        assert_eq!(res.log_folder_path, PathBuf::from("/home/steve/Data/MDR_Logs/who"));
        assert_eq!(res.target, "dummy test ICTRP.csv");
    }

    #[test]
    #[should_panic]
    fn check_503_no_csv_folder_panics() {

        let config = r#"
[data]
target_file = "dummy test ICTRP.csv"

[folders]
json_data_path="/home/steve/Data/MDR json files/who"
log_folder_path="/home/steve/Data/MDR_Logs/who"
        
[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="who"
            "#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target", "-t", "503"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let _res = get_params(cli_pars, &config_string).unwrap();
    }

    #[test]
    #[should_panic]
    fn check_503_no_target_panics() {

        let config = r#"
[data]

[folders]
csv_data_path="/home/steve/Data/MDR source data/WHO"
json_data_path="/home/steve/Data/MDR json files/who"
log_folder_path="/home/steve/Data/MDR_Logs/who"
        
[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="who"
            "#;
        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target", "-t", "503"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let _res = get_params(cli_pars, &config_string).unwrap();
    }


     #[test]
    fn check_a_flag_gives_correct_params() {

        let config = r#"
[data]
full_file_stem = "ICTRPFullExport "
full_file_num = "22"
last_file_imported = "20250106 ICTRP.csv"
target_file = "dummy test ICTRP.csv"

[folders]
csv_data_path="/home/steve/Data/MDR source data/WHO"
csv_full_path="/home/steve/Data/MDR source data/WHO/Full export 2025-02"
json_data_path="/home/steve/Data/MDR json files/who"
log_folder_path="/home/steve/Data/MDR logs/who"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5432"
mon_db_name="mon"
src_db_name="who"
        "#;

        let config_string = config.to_string();
        config_reader::populate_config_vars(&config_string).unwrap();

        let args : Vec<&str> = vec!["dummy target", "-a"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let cli_pars = cli_reader::fetch_valid_arguments(test_args).unwrap();

        let res = get_params(cli_pars, &config_string).unwrap();
        
        assert_eq!(res.dl_type, 0);
        assert_eq!(res.doing_agg_only, true);

        assert_eq!(res.csv_data_path, PathBuf::from(""));
        assert_eq!(res.csv_full_path, PathBuf::from(""));
        assert_eq!(res.json_data_path, PathBuf::from(""));
        assert_eq!(res.log_folder_path, PathBuf::from("/home/steve/Data/MDR logs/who"));
        
        assert_eq!(res.full_file_stem, "".to_string());
        assert_eq!(res.full_file_num, 0);
        assert_eq!(res.last_file_imported, "".to_string());
        assert_eq!(res.target, "".to_string());
    }
}
