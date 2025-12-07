use std::env;
use dotenvy::dotenv;
use mongodb::{Client, Database};

#[derive(Clone)]
pub struct DbConfig {
    pub mongo_uri: String,
    pub database_name: String,
}

impl DbConfig {
    fn from_env() -> Self {
        dotenv().ok();
        DbConfig{
            mongo_uri: env::var("MONGODB_URL").expect("Mongo uri must be set"),
            database_name: env::var("MONGODB_DATABASE").unwrap_or("movieland".to_string()),
        }
    }

    pub async fn create_db() -> Database {
        let config = DbConfig::from_env();
        let client = Client::with_uri_str(&config.mongo_uri)
            .await
            .expect("Failed to initialize MongoDB Client");
        client.database(&config.database_name)
    }
}
