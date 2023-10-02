# Remove all old CSVs
rm *.csv

# For each paystub txt (pay*.txt), dump the event tables into a specific CSV
Get-ChildItem .. -Filter "pay*.txt" | % { cargo run -- "$_" > "$($_.BaseName).csv"}

# Get the header from any CSV
Get-Content pay2019.csv | Select-Object -First 1 > all.csv

# Skip the header from every CSV and dump the data into the a final CSV
Get-ChildItem . -Filter "pay*.csv" | % { Get-Content "$_" | Select-Object -Skip 1 >> all.csv }