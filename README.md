N.B. STILL IN RELATIVELY EARLY STAGES OF DEVELOPMENT

<h2>Introduction</h2>
A program intended to run on a scheduled (weekly) basis, that takes CSV files obtained from the WHO ICTRP site and processes them to create:<br/> 
a) For each trial registry source, an updated summary table of studies, as downloaded so far.<br/> 
b) For most sources (current exceptions are CTG and IRSCTN) creates a .json file containing the major components of the WHO dataset, storing that within a registry specific folder.<br/> 
Existing study records, and / or json files, are simply over-written by new data.

It has two main functions. 
<ul>
<li> The first is to provide a study summary record for every study on the WHO system. This includes basic data, e.g. study title, primary and secondary ids, type, status, registration date, results date, age and gender eligibility, and countries where running. This therefore provides a global overview of registered clinical research, allowing that research to be summarised and tracked over time.</li>
<li> The second is to create a source of information about each study and its data objects, as a .json file, as the basis for later processing and incorporation into an MDR (or similar) processing pipeline.</li>
</ul>
In several cases, the WHO data can be usefully supplemented by more detailed data direct from the registry, but it provides an extremely useful starting point and for many registries is the only source of data available.<br/> 
The program also summarises the contents of each file, in terms of the number of records obtained from each source registry.

<h2>Operation</h2>
It has three modes of operation:
<ul>
<li> DL type 501: The default mode, in which the program examines a designated folder on the host machine and identifies files with the correct name pattern, dated after a given date, and processes each of them in turn. The base date is provided in the app_config file, as the name of the file most recently processed before the current run.</li>
<li> DL type 502: Processing of a 'full download' of the WHO data. Periodically, usually once or twice a year, WHO rebases the ICTRP data by releasing a large file containing <i>all</i> of the data, rather than weekly updates that include only new or changed records. This file can be broken into over 20 smaller files, of 50,000 records each, which can then be processed in sequence to recreate the whole of the WHO data in the databases / json file collections. The program needs to know where to find the relevant files but works through them automatically in sequence.</li>
<li> DL type 503: Process a single designated file. The file name must be provided, with the parent data folder being designated in the app_config.toml file. This mode is useful for testing but in normal practice DL types 501 or 502 would be used. The other DL types (501 and 502) both call this routine sequentially once they have determined the specific files to be processed.</li>
</ul>

The type is included as -t parameter in the command line, e.g. 'cargo run -r -- -t 502', or cargo run -r -- -t 503 -f "<file name>". Running the program in release mode (carg run -r) is recommended.<br/> 
Apart from -t and -f, the only other parameter is -a. <br/> 
This switches the progranm to 'aggregation' mode, in which data from the various source based WHO data tables are combined to create summary statistics and time series that can be used as the basis of graphs. If -a is run any other parameters are ignored. Successful aggregation depends on identifying studies that are registered in two or more registries, so that duplicate entries can be taken into account. This is (to be) done using the secondary id data, though inconsistencies and incompleteness of that data mean that the number of multiple registrations identified is an under-estimate of the true figure.

<h2>Set up</h2>
The program needs access to a postgres database called 'who', as well as to a monitoring database called 'mon'.<br/>
It requires an app_config file with the following fields completed:

[data]
full_file_stem = "" <br/> 
full_file_num = "" <br/> 
last_file_imported = "" <br/> 
target_file = "" <br/> 

[folders]
csv_data_path="" <br/> 
csv_full_path="" <br/> 
json_data_path="" <br/> 
log_folder_path="" <br/> 

[database]
db_host=""<br/> 
db_user=""<br/> 
db_password=""<br/> 
db_port=""<br/> 
mon_db_name=""<br/> 
src_db_name=""<br/> 

where: <br/> 
<i>full_file_stem</i> is the stem of the file names in a full file download. The stem will be suffixed by a number, as they are generated in sequence by a small PowerShell script (see below). The program in DL type 502 runs through each of the files in sequence, combining the file name in each case with the folder name where the files are stopred (from csv_full_path). <br/> 
<i>full_file_num</i> indicates the total number of files generated from the full download WHO file, and thus the limit of the processing loop in DL mode 502. <br/> 
<i>last_file_imported</i> gives the name of the csv file that the system last processed. This acts as the comparison point when finding newer files, in the default 501 mode. <br/> 
<i>target_file</i> gives the name of the specific target file whern operating in DL 503 mode, i.e. processing a single file. This file can also be specified - and usually is - as a command line parameter after the '-f' flag.

<i>csv_data_path</i> is the folder path where the 'routine', i.e. weekly update, WHO csv files are to be found.<br/> 
<i>csv_full_path</i> is the folder path where the collection of files generated from a full download are to be found.<br/> 
<i>json_data_path</i> is the parent folder for storage of the json files generated by the system. Each source registry has its own sub-folder within that path.<br/> 
<i>log_folder_path</i> is the folder for storing log files generated by the program.<br/> 

Database parameters are standard. By default, the mon_db_name is set to 'mon', the src_db_name to 'who'.<br/> 

<h3>Pre-processing of WHO files</h3>

a) For routine use: <br/> 
1) The WHO files are downloaded as .zips, that have names that reflect the date on which they were created, e.g. 'ICTRPWeek24February2025.zip'. <br/>
2) Each file should be unzipped and placed in the csv_data_path folder. Initially it normally has the same name as the zip file.<br/>
3) it <b><i>MUST</i></b> then be renamed to put the date in an ISO like format, without the dashes, followed by ' ICTRP', e.g. ICTRPWeek24February2025.csv becomes <b>20250224 ICTRP.csv</b>.<br/>
4) This renaming is essential for the mechanism identifying new files to work, because it relies on reading the file dates from their names. As a bonus, it also makes it much easier for monitoring of files as they are automatically listed in date order.<br/>
5) Once renamed the file(s) can be processd by running against -t 501.<br/>

b) For full downloads: <br/>
1) The full export zip file should be downloaded (it is usually called 'FullExport.....zip') and unzipped. The resulting file is well over 5 GB. It should be placed in a separate source folder (csv_full_path in the app_config file). <br/>
2) The size of the file makes it difficult to inspect and process. It is therefore useful to split it into a series of numbered smaller files and process these one at a time.<br/>
3) This is done by applying a small PowerShell script to the file, which reads and writes each line but generates a new file every 50,000 lines. As there are over a million registration records over 20 files are created. The whole process takes about 20 seconds. <br/>
4) Each file has the name stem provided by the script - the same as the one in the full_file_stem parameter in the config file. The config fiile should be updated to ensure the correct entries for csv_full_path, full_file_stem and full_file_num.<br/>
5) The file(s) can then be processd by running against -t 502.<br/>
<br/>
The download script is included in the code in the utilities folder, for convenience. The easiest way to apply it is to open a PowerShell ISE and load it, modifying it if necessary, and run it from there. As PowerShell it is Windows specific but should be readily adapted to an equivalent bash script if required.
<br/>
N.B. STILL IN RELATIVELY EARLY STAGES OF DEVELOPMENT
