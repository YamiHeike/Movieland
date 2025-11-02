use serde::{ Serialize, Deserialize };
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Genre {
    pub id: Option<Uuid>,
    pub name: String,
}