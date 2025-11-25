use std::env;
use dotenvy::dotenv;

pub struct DbConfig {
    pub mongo_uri: String,
    pub database_name: String,
}

impl DbConfig {
    pub fn from_env() -> Self {
        dotenv().ok();
        DbConfig{
            mongo_uri: env::var("MONGODB_URL").expect("Mongo uri must be set"),
            database_name: env::var("MONGODB_DATABASE").unwrap_or("movieland".to_string()),
        }
    }
}
