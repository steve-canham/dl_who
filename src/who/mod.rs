mod file_model;
mod processor;
pub mod who_helper;
pub mod gen_helper;

use std::path::PathBuf;
use crate::{AppError, DownloadResult};
use crate::data::update_who_study_mon;
use file_model::WHOLine;
use std::fs;
use std::io::BufReader;
use std::fs::File;
use csv::ReaderBuilder;
use std::io::Write;
use serde_json::to_string_pretty;
use sqlx::{Pool, Postgres};

#[allow(dead_code)]
pub struct WhoDLRes {
    pub source: i32,
    pub number_dl: i32,
}

pub async fn process_single_file(file_path: &PathBuf, json_path: &PathBuf, res: &mut DownloadResult, dl_id: i32, pool: &Pool<Postgres>) -> Result<Vec<WhoDLRes>, AppError> {

    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let mut csv_rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(buf_reader);
    
    let mut i = 0;

    // set up vectors, counters here
    // set up json writers here (?)

     for result in csv_rdr.deserialize() {
    
        let who_line: WHOLine = result?;
        res.num_records_checked +=1;

        let who_rec = match processor::process_line(who_line, i) {
            Some(r) => r,
            None => {continue;},
        };

        // Work out file destination folder and full path 
        
        let db_name = &who_rec.db_name;
        let file_folder: PathBuf = [json_path, &PathBuf::from(db_name)].iter().collect();
        if !folder_exists(&file_folder) {
            fs::create_dir_all(&file_folder)?;
        }

        let sd_sid = &who_rec.sd_sid;
        let file_name = format!("{}.json", sd_sid);
        let full_path: PathBuf = [file_folder, PathBuf::from(&file_name)].iter().collect();

        // Write the JSON string to a file
        // update per source counters
        // update database (relevant DB's source record)
        // see if it is a new download, or an existing one
        // update res

        let json_string = to_string_pretty(&who_rec).unwrap();
        let mut file = File::create(&full_path)?;
        file.write_all(json_string.as_bytes())?;
        
        let added = update_who_study_mon(&db_name, &sd_sid, &who_rec.remote_url, dl_id,
                        &who_rec.record_date, &full_path, pool).await?;

        res.num_records_downloaded +=1;
        if added {
            res.num_records_added +=1;
        } 
                    
        i += 1;
        //if i > 10 {
        //    break;
        //}
    }
    
    //
    // If all successful
    // Take the various counters and store them in the database
    // return the aggregate figures in the res struct ... 
    // record the event in the dl events table 
 
    Ok(vec![WhoDLRes {
        source: 10000,
        number_dl: 0,
    }])

}

fn folder_exists(folder_name: &PathBuf) -> bool {
    let res = match folder_name.try_exists() {
        Ok(true) => true,
        Ok(false) => false, 
        Err(_e) => false,           
    };
    res
}



/*
        DownloadResult res = new();
        string? file_base = source.local_folder;

        if (file_base is null)
        {
            _loggingHelper.LogError("Null value passed for local folder value for this source");
            return res;   // return zero result
        }

        WHO_Processor who_processor = new();
        string source_file = opts.FileName!;     // already checked as non-null

        var json_options = new JsonSerializerOptions()
        {
            Encoder = JavaScriptEncoder.UnsafeRelaxedJsonEscaping,
            WriteIndented = true
        };

                WHORecord? r = who_processor.ProcessStudyDetails(sr);

                if (r is not null)
                {
                    // Write out study record as JSON, log the download.

                    if (!string.IsNullOrEmpty(r.db_name))
                    {
                        string folder_name = _loggingHelper.DataFolderPath + r.db_name + @"\";
                        if (!Directory.Exists(folder_name))
                        {
                            Directory.CreateDirectory(folder_name);
                        }
                        string file_name = r.sd_sid + ".json";
                        string full_path = Path.Combine(folder_name, file_name);
                        try
                        {
                            await using FileStream jsonStream = File.Create(full_path);
                            await JsonSerializer.SerializeAsync(jsonStream, r, json_options);
                            await jsonStream.DisposeAsync();
                            
                            if (_monDataLayer.IsWHOTestStudy(r.db_name, r.sd_sid))
                            {
                                // write out copy of the file in the test folder

                                string test_path = _loggingHelper.TestFilePath;
                                string full_test_path = Path.Combine(test_path, file_name);
                                await using FileStream jsonStream2 = File.Create(full_test_path);
                                await JsonSerializer.SerializeAsync(jsonStream2, r, json_options);
                                await jsonStream2.DisposeAsync();
                            }
                        }
                        catch (Exception e)
                        {
                            _loggingHelper.LogLine("Error in trying to save file at " + full_path + ":: " + e.Message);
                        }

                        bool added = _monDataLayer.UpdateWhoStudyLog(r.db_name, r.sd_sid, r.remote_url, (int)opts.dl_id!,
                                                            r.record_date?.FetchDateTimeFromISO(), full_path);
                        res.num_downloaded++;
                        if (added) res.num_added++;
                    }
                }

                if (res.num_checked % 100 == 0) _loggingHelper.LogLine(res.num_checked.ToString());
            }
        }
        return res;
    }
}


*/