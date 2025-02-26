
use std::sync::OnceLock;
use toml;
use serde::Deserialize;
use crate::err::AppError;
use std::path::PathBuf;


#[derive(Debug, Deserialize)]
pub struct TomlConfig {
    pub data: Option<TomlDataPars>,
    pub folders: Option<TomlFolderPars>, 
    pub database: Option<TomlDBPars>,
}

#[derive(Debug, Deserialize)]
pub struct TomlDataPars {
    pub full_file_stem: Option<String>,
    pub full_file_num: Option<String>,
    pub last_file_imported: Option<String>,
    pub target_file: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TomlFolderPars {
    pub csv_data_path: Option<String>,
    pub csv_full_path: Option<String>,
    pub json_data_path: Option<String>,
    pub log_folder_path: Option<String>,

}

#[derive(Debug, Deserialize)]
pub struct TomlDBPars {
    pub db_host: Option<String>,
    pub db_user: Option<String>,
    pub db_password: Option<String>,
    pub db_port: Option<String>,
    pub db_name: Option<String>,
}


pub struct Config {
    pub data_details: DataPars, 
    pub folders: FolderPars, 
    pub db_pars: DBPars,
}

pub struct DataPars {
    pub full_file_stem: String,
    pub full_file_num: usize,
    pub last_file_imported: String,
    pub target_file: String,
}

pub struct FolderPars {
    pub csv_data_path: PathBuf,
    pub csv_full_path: PathBuf,
    pub json_data_path: PathBuf,
    pub log_folder_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct DBPars {
    pub db_host: String,
    pub db_user: String,
    pub db_password: String,
    pub db_port: usize,
    pub db_name: String,
}

pub static DB_PARS: OnceLock<DBPars> = OnceLock::new();

pub fn populate_config_vars(config_string: &String) -> Result<Config, AppError> {

    let toml_config = toml::from_str::<TomlConfig>(&config_string)
        .map_err(|_| {AppError::ConfigurationError("Unable to parse config file.".to_string(),
                                       "File (app_config.toml) may be malformed.".to_string())})?;

    let toml_data_details = match toml_config.data {
        Some(d) => d,
        None => {return Result::Err(AppError::ConfigurationError("Missing or misspelt configuration section.".to_string(),
        "Cannot find a section called '[data]'.".to_string()))},
    };

    let toml_database = match toml_config.database {
        Some(d) => d,
        None => {return Result::Err(AppError::ConfigurationError("Missing or misspelt configuration section.".to_string(),
            "Cannot find a section called '[database]'.".to_string()))},
    };

    let toml_folders = match toml_config.folders {
        Some(f) => f,
        None => {return Result::Err(AppError::ConfigurationError("Missing or misspelt configuration section.".to_string(),
           "Cannot find a section called '[folders]'.".to_string()))},
    };
       
    let config_folders = verify_folder_parameters(toml_folders)?;
    let config_data_dets = verify_data_parameters(toml_data_details)?;
    let config_db_pars = verify_db_parameters(toml_database)?;

    let _ = DB_PARS.set(config_db_pars.clone());

    Ok(Config{
        data_details: config_data_dets,
        folders: config_folders,
        db_pars: config_db_pars,
    })
}

fn verify_data_parameters(toml_data_pars: TomlDataPars) -> Result<DataPars, AppError> {

    let full_file_stem = check_defaulted_string (toml_data_pars.full_file_stem, "full DL file stem", "ICTRPFullExport ", "ICTRPFullExport ");

    let full_file_num_as_string = check_defaulted_string (toml_data_pars.full_file_num, "full DL file num", "zero", "0");
    let full_file_num: usize = full_file_num_as_string.parse().unwrap_or_else(|_| 0);
    
    let last_file_imported = check_defaulted_string (toml_data_pars.last_file_imported, "last file imported", "empty string", "");

    let target_file = check_defaulted_string (toml_data_pars.target_file, "single target file", "empty string", "");
        
    Ok(DataPars {
        full_file_stem,
        full_file_num,
        last_file_imported,
        target_file,
    })
}

fn verify_folder_parameters(toml_folders: TomlFolderPars) -> Result<FolderPars, AppError> {

    let csv_data_path_string = check_defaulted_string (toml_folders.csv_data_path, "csv data path", "csv_data_path", "");

    let csv_full_path_string = check_defaulted_string (toml_folders.csv_full_path, "csv full download path", "csv_data_path", &csv_data_path_string);

    let json_data_path_string = check_essential_string (toml_folders.json_data_path, "json outputs parents folder", "json_data_path")?;

    let log_folder_path_string = check_essential_string (toml_folders.log_folder_path, "log folder", "log_folder_path")?;

    Ok(FolderPars {
        csv_data_path: PathBuf::from(csv_data_path_string),
        csv_full_path: PathBuf::from(csv_full_path_string),
        json_data_path: PathBuf::from(json_data_path_string),
        log_folder_path: PathBuf::from(log_folder_path_string),
    })
}

fn verify_db_parameters(toml_database: TomlDBPars) -> Result<DBPars, AppError> {

    // Check user name and password first as there are no defaults for these values.
    // They must therefore be present.

    let db_user = check_essential_string (toml_database.db_user, "database user name", "db_user")?; 

    let db_password = check_essential_string (toml_database.db_password, "database user password", "db_password")?;
       
    let db_host = check_defaulted_string (toml_database.db_host, "DB host", "localhost", "localhost");
            
    let db_port_as_string = check_defaulted_string (toml_database.db_port, "DB port", "5432", "5432");
    let db_port: usize = db_port_as_string.parse().unwrap_or_else(|_| 5432);

    let db_name = check_defaulted_string (toml_database.db_name, "DB name", "mon", "mon");

    Ok(DBPars {
        db_host,
        db_user,
        db_password,
        db_port,
        db_name,
    })
}


fn check_essential_string (src_name: Option<String>, value_name: &str, config_name: &str) -> Result<String, AppError> {
 
    let s = match src_name {
        Some(s) => s,
        None => "none".to_string(),
    };

    if s == "none".to_string() || s.trim() == "".to_string()
    {
        return Result::Err(AppError::ConfigurationError("Essential configuration value missing or misspelt.".to_string(),
        format!("Cannot find a value for {} ({}).", value_name, config_name)))
    }
    else {
        Ok(s)
    }
}


fn check_defaulted_string (src_name: Option<String>, value_name: &str, default_name: &str, default:  &str) -> String {
 
    let s = match src_name {
        Some(s) => s,
        None => "none".to_string(),
    };

    if s == "none".to_string() || s.trim() == "".to_string()
    {
        println!("No value found for the {} in config file - using the provided default value ('{}') instead.", 
        value_name, default_name);
        default.to_owned()
    }
    else {
       s
    }
}


pub fn fetch_db_name() -> Result<String, AppError> {
    let db_pars = match DB_PARS.get() {
         Some(dbp) => dbp,
         None => {
            return Result::Err(AppError::MissingDBParameters());
        },
    };
    Ok(db_pars.db_name.clone())
}

pub fn fetch_db_conn_string(db_name: &String) -> Result<String, AppError> {
    let db_pars = match DB_PARS.get() {
         Some(dbp) => dbp,
         None => {
            return Result::Err(AppError::MissingDBParameters());
        },
    };
    
    Ok(format!("postgres://{}:{}@{}:{}/{}", 
    db_pars.db_user, db_pars.db_password, db_pars.db_host, db_pars.db_port, db_name))
}



#[cfg(test)]
mod tests {
    use super::*;

    // Ensure the parameters are being correctly extracted from the config file string
    
    #[test]
    fn check_config_with_all_params_present() {

        let config = r#"
[data]
full_file_stem = "ICTRPFullExport "
full_file_num = "22"
last_file_imported = "20250106 ICTRP.csv"
target_file = "20250210 ICTRP.csv"

[folders]
csv_data_path="E:/MDR source data/WHO/data"
csv_full_path="E:/MDR source data/WHO/data/Full export 2025-02"
json_data_path="E:/MDR source files"
log_folder_path="E:/MDR/MDR Logs"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"
db_name="mon"
"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();

        assert_eq!(res.folders.csv_data_path, PathBuf::from("E:/MDR source data/WHO/data"));
        assert_eq!(res.folders.csv_full_path, PathBuf::from("E:/MDR source data/WHO/data/Full export 2025-02"));
        assert_eq!(res.folders.json_data_path, PathBuf::from("E:/MDR source files"));
        assert_eq!(res.folders.log_folder_path, PathBuf::from("E:/MDR/MDR Logs"));

        assert_eq!(res.data_details.full_file_stem, "ICTRPFullExport ");
        assert_eq!(res.data_details.full_file_num, 22);
        assert_eq!(res.data_details.last_file_imported, "20250106 ICTRP.csv");
        assert_eq!(res.data_details.target_file, "20250210 ICTRP.csv");

        assert_eq!(res.db_pars.db_host, "localhost");
        assert_eq!(res.db_pars.db_user, "user_name");
        assert_eq!(res.db_pars.db_password, "password");
        assert_eq!(res.db_pars.db_port, 5433);
        assert_eq!(res.db_pars.db_name, "mon");
    }
    

    #[test]
    fn check_config_with_win_folders() {

        let config = r#"

[data]
full_file_stem = "ICTRPFullExport "
full_file_num = "22"
last_file_imported = "20250106 ICTRP.csv"
target_file = "20250210 ICTRP.csv"

[folders]
csv_data_path="E:\\MDR source data\\WHO\\data"
csv_full_path="E:\\MDR source data\\WHO\\data\\Full export 2025-02"
json_data_path="E:\\MDR source files"
log_folder_path="E:\\MDR\\MDR Logs"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"
db_name="mon"
"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();

        assert_eq!(res.folders.csv_data_path, PathBuf::from("E:\\MDR source data\\WHO\\data"));
        assert_eq!(res.folders.csv_full_path, PathBuf::from("E:\\MDR source data\\WHO\\data\\Full export 2025-02"));
        assert_eq!(res.folders.json_data_path, PathBuf::from("E:\\MDR source files"));
        assert_eq!(res.folders.log_folder_path, PathBuf::from("E:\\MDR\\MDR Logs"));

        assert_eq!(res.data_details.full_file_stem, "ICTRPFullExport ");
        assert_eq!(res.data_details.full_file_num, 22);
        assert_eq!(res.data_details.last_file_imported, "20250106 ICTRP.csv");
        assert_eq!(res.data_details.target_file, "20250210 ICTRP.csv");

        assert_eq!(res.db_pars.db_host, "localhost");
        assert_eq!(res.db_pars.db_user, "user_name");
        assert_eq!(res.db_pars.db_password, "password");
        assert_eq!(res.db_pars.db_port, 5433);
        assert_eq!(res.db_pars.db_name, "mon");
    }


    #[test]
    fn check_config_with_missing_csv_folder() {

        let config = r#"

[data]
full_file_stem = "ICTRPFullExport "
full_file_num = "22"
last_file_imported = "20250106 ICTRP.csv"
target_file = "20250210 ICTRP.csv"

[folders]
csv_full_path="E:\\MDR source data\\WHO\\data\\Full export 2025-02"
json_data_path="E:\\MDR source files"
log_folder_path="E:\\MDR\\MDR Logs"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"
db_name="mon"
"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();

        assert_eq!(res.folders.csv_data_path, PathBuf::from(""));
        assert_eq!(res.folders.csv_full_path, PathBuf::from("E:/MDR source data/WHO/data/Full export 2025-02"));
        assert_eq!(res.folders.json_data_path, PathBuf::from("E:/MDR source files"));
        assert_eq!(res.folders.log_folder_path, PathBuf::from("E:/MDR/MDR Logs"));

    }


    #[test]
    fn check_config_with_missing_csv_full_folder() {

        let config = r#"

[data]
full_file_stem = "ICTRPFullExport "
full_file_num = "22"
last_file_imported = "20250106 ICTRP.csv"
target_file = "20250210 ICTRP.csv"

[folders]
csv_data_path="E:/MDR source data/WHO/data"
json_data_path="E:/MDR source files"
log_folder_path="E:/MDR/MDR Logs"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"
db_name="mon"
"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();

        assert_eq!(res.folders.csv_data_path, PathBuf::from("E:/MDR source data/WHO/data"));
        assert_eq!(res.folders.csv_full_path, PathBuf::from("E:/MDR source data/WHO/data"));
        assert_eq!(res.folders.json_data_path, PathBuf::from("E:/MDR source files"));
        assert_eq!(res.folders.log_folder_path, PathBuf::from("E:/MDR/MDR Logs"));
        
    }


    #[test]
    #[should_panic]
    fn check_panics_if_missing_json_folder () {

        let config = r#"

[data]
full_file_stem = "ICTRPFullExport "
full_file_num = "22"
last_file_imported = "20250106 ICTRP.csv"
target_file = "20250210 ICTRP.csv"

[folders]
csv_data_path="E:\\MDR source data\\WHO\\data"
csv_full_path="E:\\MDR source data\\WHO\\data\\Full export 2025-02"
log_folder_path="E:\\MDR\\MDR Logs"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"
db_name="mon"
"#;
        let config_string = config.to_string();
        let _res = populate_config_vars(&config_string).unwrap();
    }


    #[test]
    #[should_panic]
    fn check_panics_if_missing_log_folder () {

        let config = r#"

[data]
full_file_stem = "ICTRPFullExport "
full_file_num = "22"
last_file_imported = "20250106 ICTRP.csv"
target_file = "20250210 ICTRP.csv"

[folders]
csv_data_path="E:\\MDR source data\\WHO\\data"
csv_full_path="E:\\MDR source data\\WHO\\data\\Full export 2025-02"
json_data_path="E:\\MDR source files"
log_folder_path=""

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"
db_name="mon"
"#;
        let config_string = config.to_string();
        let _res = populate_config_vars(&config_string).unwrap();
    }
    

    #[test]
    fn check_missing_data_details_become_defaults_or_empty_strings() {

        let config = r#"
[data]

[folders]
csv_data_path="E:/MDR source data/WHO/data"
csv_full_path="E:/MDR source data/WHO/data/Full export 2025-02"
json_data_path="E:/MDR source files"
log_folder_path="E:/MDR/MDR Logs"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"
db_name="mon"
"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();

        assert_eq!(res.folders.csv_data_path, PathBuf::from("E:/MDR source data/WHO/data"));
        assert_eq!(res.folders.csv_full_path, PathBuf::from("E:/MDR source data/WHO/data/Full export 2025-02"));
        assert_eq!(res.folders.json_data_path, PathBuf::from("E:/MDR source files"));
        assert_eq!(res.folders.log_folder_path, PathBuf::from("E:/MDR/MDR Logs"));

        assert_eq!(res.data_details.full_file_stem, "ICTRPFullExport ");
        assert_eq!(res.data_details.full_file_num, 0);
        assert_eq!(res.data_details.last_file_imported, "");
        assert_eq!(res.data_details.target_file, "");

        assert_eq!(res.db_pars.db_host, "localhost");
        assert_eq!(res.db_pars.db_user, "user_name");
        assert_eq!(res.db_pars.db_password, "password");
        assert_eq!(res.db_pars.db_port, 5433);
        assert_eq!(res.db_pars.db_name, "mon");
    }



    #[test]
    #[should_panic]
    fn check_missing_user_name_panics() {

        let config = r#"

[data]
full_file_stem = "ICTRPFullExport "
full_file_num = "22"
last_file_imported = "20250106 ICTRP.csv"
target_file = "20250210 ICTRP.csv"

[folders]
csv_data_path="E:/MDR source data/WHO/data"
csv_full_path="E:/MDR source data/WHO/data/Full export 2025-02"
json_data_path="E:/MDR source files"
log_folder_path="E:/MDR/MDR Logs"

[database]
db_host="localhost"
db_password="password"
db_port="5433"
db_name="mon"
"#;
        let config_string = config.to_string();
        let _res = populate_config_vars(&config_string).unwrap();
    }


    #[test]
    fn check_db_defaults_are_supplied() {

        let config = r#"

[data]
full_file_stem = "ICTRPFullExport "
full_file_num = "22"
last_file_imported = "20250106 ICTRP.csv"
target_file = "20250210 ICTRP.csv"

[folders]
csv_data_path="E:/MDR source data/WHO/data"
csv_full_path="E:/MDR source data/WHO/data/Full export 2025-02"
json_data_path="E:/MDR source files"
log_folder_path="E:/MDR/MDR Logs"

[database]
db_user="user_name"
db_password="password"

"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();

        assert_eq!(res.db_pars.db_host, "localhost");
        assert_eq!(res.db_pars.db_user, "user_name");
        assert_eq!(res.db_pars.db_password, "password");
        assert_eq!(res.db_pars.db_port, 5432);
        assert_eq!(res.db_pars.db_name, "mon");
    }


#[test]
    fn missing_port_gets_default() {

        let config = r#"

[data]
full_file_stem = "ICTRPFullExport "
full_file_num = "22"
last_file_imported = "20250106 ICTRP.csv"
target_file = "20250210 ICTRP.csv"

[folders]
csv_data_path="E:/MDR source data/WHO/data"
csv_full_path="E:/MDR source data/WHO/data/Full export 2025-02"
json_data_path="E:/MDR source files"
log_folder_path="E:/MDR/MDR Logs"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_name="mon"

"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();

        assert_eq!(res.db_pars.db_host, "localhost");
        assert_eq!(res.db_pars.db_user, "user_name");
        assert_eq!(res.db_pars.db_password, "password");
        assert_eq!(res.db_pars.db_port, 5432);
        assert_eq!(res.db_pars.db_name, "mon");
    }

}
  

