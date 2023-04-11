use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    mongo: MongoConfig,    
}

#[derive(Deserialize)]
struct MongoConfig {
    uri: String,
    local_uri: String,
}

impl Config {
    pub fn new(config_path: &str) -> Self {
        let pathway = config_path;
        let config_file = std::fs::read_to_string(pathway).unwrap();
        let config: Config = toml::from_str(&config_file).unwrap();

        config
    }

    pub fn get_mongo_uri(&self) -> &str {
        &self.mongo.uri
    }

    pub fn get_local_mongo_uri(&self) -> &str {
        &self.mongo.local_uri
    }
}
