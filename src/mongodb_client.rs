use crate::config::Config;
// use crate::models::{Rebate, Sale};

use futures::TryStreamExt;
use mongodb::{Client, Cursor, options::ClientOptions, Database, Collection, bson::{doc, Document}};
use anyhow::Result;


pub struct Mongo {
    client: Client,
    dev_client: Client,
}

impl Mongo {
    pub async fn new(config: &Config) -> Result<Self, mongodb::error::Error> {        
        let mut client_options: ClientOptions = ClientOptions::parse(config.get_mongo_uri()).await?;
        client_options.app_name = Some("atlas".to_string());
        let client: Client = Client::with_options(client_options)?;

        let mut dev_client_options: ClientOptions = ClientOptions::parse(config.get_local_mongo_uri()).await?;
        dev_client_options.app_name = Some("local".to_string());
        let dev_client: Client = Client::with_options(dev_client_options)?;

        Ok(Self { client, dev_client })
    }

    pub async fn archive(&self, database: &str, collection: &str, overwrite: &bool) -> Result<(), mongodb::error::Error> {
        let db: Database = self.client.database(database);
        let coll: Collection<Document> = db.collection(collection);

        let dev_db: Database = self.dev_client.database(database);
        let dev_coll: Collection<Document> = dev_db.collection(collection);

        let cursor: Cursor<Document> = coll.find(None, None).await?;

        let to_be_inserted: Vec<Document> = cursor.try_collect().await?;      

        if to_be_inserted.len() > 0 && *overwrite {
            let collections = dev_db.list_collection_names(doc!{}).await?;

            if collections.contains(&collection.to_string()) {
                dbg!(format!("dropping collection={}", collection));                
                dev_coll.delete_many(doc!{}, None).await?;
            };

            dbg!(format!("inserting {} documents to collection={}", to_be_inserted.len(), collection));
            
            dev_coll.insert_many(&to_be_inserted, None).await?;
            
            dbg!(format!("done archiving {} documents in collection={}", to_be_inserted.len(), collection));
        } else {
            dbg!(format!("{} documents available in collection={}", to_be_inserted.len(), collection));
        }       

        Ok(())
    }

    pub async fn collection_exists(&self,database: &str, collection: &str) -> Result<bool, mongodb::error::Error> {
        let db: Database = self.client.database(database);
        let collections = db.list_collection_names(doc!{}).await?;

        Ok(collections.contains(&collection.to_string()))
    }
}

