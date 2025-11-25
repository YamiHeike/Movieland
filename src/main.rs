mod db_config;

use dotenvy::dotenv;
use mongodb::bson::DateTime;
use uuid::Uuid;
use crate::models::Movie;
use crate::services::movie_service::create_movie;

mod models;
mod services;

#[tokio::main]
async fn main() {

}
