use clap::{command, Arg, ArgMatches};
use crate::err::AppError;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CliPars {
    pub dl_type: usize,
    pub target_file: String,
}

pub fn fetch_valid_arguments(args: Vec<OsString>) -> Result<CliPars, AppError>
{ 
    let parse_result = parse_args(args.to_vec())?;

    // These parameters guaranteed to unwrap OK as all have a default value;

    let dl_type_as_string = parse_result.get_one::<String>("dl_type").unwrap();
    let dl_type: usize = dl_type_as_string.parse().unwrap_or_else(|_| 0);

    let target_file = parse_result.get_one::<String>("file").unwrap();

    Ok(CliPars {
        dl_type: dl_type,
        target_file: target_file.clone(),
    }) 
}


pub fn config_file_exists()-> bool {
    let config_path = PathBuf::from("./app_config.toml");
    let res = match config_path.try_exists() {
        Ok(true) => true,
        Ok(false) => false, 
        Err(_e) => false,           
    };
    res
}


pub fn get_initalising_cli_pars()  -> CliPars {
    
    CliPars {
        dl_type: 501,
        target_file: "".to_string(),
    }
}


fn parse_args(args: Vec<OsString>) -> Result<ArgMatches, clap::Error> {

    command!()
        .about("Imports data from ROR json file (v2) and imports it into a database")
        .arg(
             Arg::new("dl_type")
            .short('t')
            .long("type")
            .help("An integer indicating the type of download required")
            .default_value("501")
        )
        .arg(
            Arg::new("file")
           .short('f')
           .long("file")
           .required(false)
           .help("A string with the target file name")
           .default_value("")
        )
    .try_get_matches_from(args)

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_cli_no_explicit_params() {
        let target = "dummy target";
        let args: Vec<&str> = vec![target];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.target_file, "");
        assert_eq!(res.dl_type, 501);
        
    }
  
    #[test]
    fn check_cli_with_a_dl_type() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-t", "502"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.target_file, "");
        assert_eq!(res.dl_type, 502);
    }

    #[test]
    fn check_cli_with_target() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-f", "dummy file.csv"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.target_file, "dummy file.csv");
        assert_eq!(res.dl_type, 501);
    }

    #[test]
    fn check_cli_with_dl_type_and_target() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-t", "503", "-f", "dummy file.csv"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();

        assert_eq!(res.target_file, "dummy file.csv");
        assert_eq!(res.dl_type, 503);
    }
   
}

