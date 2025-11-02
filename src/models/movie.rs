use mongodb::bson::DateTime;
use serde::{Serialize, Deserialize };
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Movie {
    pub id: Option<Uuid>,
    pub title: String,
    pub released_on: DateTime,
    pub genre_id: Uuid,
    pub actors: Vec<Uuid>,
}