

$sw = new-object System.Diagnostics.Stopwatch
$sw.Start()

# $filename is the full path to the source unzipped full export file.
# $rootName is the full path of the folder in which the generated split files will be placed.
# The last part of this path represents the file name stem for the generated files.
# Examples are as below.

$filename = "E:/MDR source data/WHO/data/Full export 2025-02/ICTRP-full-1074705.csv"
$rootName = "E:/MDR source data/WHO/data/Full export 2025-02/ICTRPFullExport "
$ext = "csv"

$linesperFile = 50000  #50k
$filecount = 1
$reader = $null
try{
    $reader = [io.file]::OpenText($filename)
    try{
        "Creating file number $filecount"
        $writer = [io.file]::CreateText("{0}{1}.{2}" -f ($rootName, $filecount.ToString("000"),$ext))
        $filecount++
        $linecount = 0

        while($reader.EndOfStream -ne $true) {

            "Reading $linesperFile lines"

            while( ($linecount -lt $linesperFile) -and ($reader.EndOfStream -ne $true)){
                $writer.WriteLine($reader.ReadLine());
                $linecount++
            }

            if($reader.EndOfStream -ne $true) {
                "Closing file"
                $writer.Dispose();

                "Creating file number $filecount"
                $writer = [io.file]::CreateText("{0}{1}.{2}" -f ($rootName,$filecount.ToString("000"),$ext))
                $filecount++
                $linecount = 0
            }
        }
    } 
    finally {
        $writer.Dispose();
    }
} finally {
    $reader.Dispose();
}
$sw.Stop()

Write-Host "Split complete in " $sw.Elapsed.TotalSeconds "seconds"