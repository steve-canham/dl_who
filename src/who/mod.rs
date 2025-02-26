mod file_model;
mod processor;
mod helper;
mod gen_helper;

use std::path::PathBuf;
use crate::{AppError, DownloadResult};
use file_model::WHOLine;
use std::io::BufReader;
use std::fs::File;
use csv::ReaderBuilder;


#[allow(dead_code)]
pub struct WhoDLRes {
    pub source: i32,
    pub number_dl: i32,
}

pub fn process_single_file(file_path: &PathBuf, _json_path: &PathBuf, _res: &DownloadResult) -> Result<Vec<WhoDLRes>, AppError> {

    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);
    let mut csv_rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(buf_reader);
    
    let mut i = 0;
 
    for result in csv_rdr.deserialize() {
    
        let source: WHOLine = result?;
        let sd_id = source.trial_id;

        println!("{}", sd_id);
        

        i += 1;
        if i > 20 {
            break;
        }
    }
    
    // for each line...
        // tries to read it as a struct
        // processes the struct to form a json file
        // gets the file name and the destination folder
        // writes the file
        // sees if it is a new download, or an existing one
        // update the relevant DB's source table to keep the records up to date
        // increments a counter for the relevant source
    //
    // If all successful
    // Take the various counters and store them in the database
    // return the aggregate figures in the res struct ... 
    // record the event in the dl events table 
    // Record that file's download - source file, overall numbers, date etc. in the database if not already covered

       

    Ok(vec![WhoDLRes {
        source: 10000,
        number_dl: 0,
    }])

}






// WHO processing unusual in that it is from a csv file
// The program loops through the file and creates a JSON file from each row
// It then distributes it to the correct source folder for 
// later harvesting.

// In some cases the file will be one of a set created from a large
// 'all data' download, in other cases it will be a weekly update file
// In both cases any existing JSON files of the same name should be overwritten.

// Download result struct here

// get source data so that local files are known for each source

// if normal download get file, from command line
// or work out files not yet processed and the order they are erquired from the record of past downloads

// if multiple files involved, as in a full data download, use data in config file to work
// through the files inthe correct order

// implies 3 flags - -s, -d, -f respectively

// set up the csv reader



 // ********************************
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

        var csv_reader_config = new CsvConfiguration(CultureInfo.InvariantCulture)
        {
            HasHeaderRecord = false,
        };

        var json_options = new JsonSerializerOptions()
        {
            Encoder = JavaScriptEncoder.UnsafeRelaxedJsonEscaping,
            WriteIndented = true
        };

        using (var reader = new StreamReader(source_file, true))
        {
            using var csv = new CsvReader(reader, csv_reader_config);
            var records = csv.GetRecords<WHO_SourceRecord>();
            _loggingHelper.LogLine("Rows loaded into WHO record structure");

            // Consider each study row in turn and turn it into a WHO record class.

            foreach (WHO_SourceRecord sr in records)
            {
                res.num_checked++;
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