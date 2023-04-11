# busse-atlas-localread
busse local read replicator to local docker mongodb

## Build
`cargo build --release && ./target/release/busse-atlas-localread.exe -a true`

## Usage
### args
```
--path -p => (string), file path to config.toml <opt>
ex) ./target/release/busse-atlas-localread.exe -p "c:\temp\global\config.toml" ... (rest)

[method.0]
--all -a => (true|false), helper flag to backup all standard collections <opt>  
ex) ./target/release/busse-atlas-localread.exe -a true

[method.1]
--collection -c => (string), [tracings, roster, data_warehouse, sales] are available standard collections
--overwrite -o => (true|false), overwrite local docker collection or get len
ex) ./target/release/busse-atlas-localread.exe -o true -c <sales|tracings|roster|data_warehouse>

[method.2]
--set_database -s => (true|false), if you want to backup a non-standard collection this is required <opt>
--database -d => (string), if backing up a non-standard collection this is required <opt>
--collection -c => (string), if backing up a non-standard collection this is required <opt>
ex) ./target/release/busse-atlas-localread.exe -s true -d busseshipping -c edi
```
