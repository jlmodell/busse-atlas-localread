mod config; mod mongodb_client; mod models;

use std::panic;

use clap::Parser;

const REBATE_DB: &str = "busserebatetraces";
const TRACINGS_COLL: &str = "tracings";
const ROSTER_COLL: &str = "roster";
const DATA_WAREHOUSE_COLL: &str = "data_warehouse";

const SALE_DB: &str = "busse_sales_data_warehouse";
const SALE_COLL: &str = "sales";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value = "C:\\temp\\global\\config.toml")]
    path: String,   
    #[clap(short, long, value_parser, default_value = "false")]
    overwrite: String,

    #[clap(short, long, value_parser, default_value = "false")]
    all: String,

    #[clap(short, long, value_parser, default_value = "false")]
    set_database: String,
    #[clap(short, long, value_parser, default_value = "")]
    database: String,

    #[clap(short, long, value_parser, default_value = "")]
    collection: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();

    let config: config::Config = config::Config::new(&args.path); // depends on args

    let mongo: mongodb_client::Mongo = mongodb_client::Mongo::new(&config).await.unwrap();  // depends on args

    let archive_all: bool = match args.all.as_str() {
        "true" => true,
        "false" => false,
        _ => panic!("invalid all option. valid options are: true, false")
    };  // depends on args

    let mut overwrite: bool = match args.overwrite.as_str() {
        "true" => true,
        "false" => false,        
        _ => panic!("invalid overwrite option. valid options are: true, false")
    };  // depends on args
    

    if archive_all == true { // if --all=<true|false> (-a) is passed as args - all of the basic collections are replicated
        overwrite = true;
        println!("overwrite={}", overwrite);
        
        let to_be_archived: Vec<&str> = vec!(TRACINGS_COLL, SALE_COLL, DATA_WAREHOUSE_COLL, ROSTER_COLL);

        for coll in to_be_archived {
            mongo.archive(match coll {                
                SALE_COLL => SALE_DB,
                _ => REBATE_DB,                
            }, coll, &overwrite).await.unwrap();
        }        
    } else { // else --collection=<string> (-c) is required; --database=<string> (-d) && --set_database=<true> (-s) is required for a non-standard collection
        println!("overwrite={}", overwrite);

        let db: &str = match args.set_database.as_str() {
            "true" => args.database.as_str(),
            "false" => match args.collection.as_str() {
                "sales" => SALE_DB,
                "tracings" => REBATE_DB,
                "roster" => REBATE_DB,
                "data_warehouse" => REBATE_DB,            
                _ => {
                    println!("collection={}", args.collection);
                    panic!("if set_database is false, database must be pre-mapped, else set_database explicitly");
                }
            },
            _ => panic!("invalid set_database option. valid options are: true, false")
        };

        match args.collection.as_str() {
            "tracings" => mongo.archive(db, TRACINGS_COLL, &overwrite).await.unwrap(),
            "sales" => mongo.archive(db, SALE_COLL, &overwrite).await.unwrap(),
            "roster" => mongo.archive(db, ROSTER_COLL, &overwrite).await.unwrap(),
            "data_warehouse" => mongo.archive(db, DATA_WAREHOUSE_COLL, &overwrite).await.unwrap(),
            _ => {
                match mongo.collection_exists(db, &args.collection).await.unwrap() {
                    true => mongo.archive(db, &args.collection, &overwrite).await.unwrap(),
                    false => {
                        println!("collection={} does not exist in database={}", args.collection, db);                    
                        panic!("collection={} does not exist in database={}", &args.collection, db);
                    }
                }
            }
        }
    }

    Ok(())
}