use serde::{Serialize, Deserialize };
use mongodb::bson::Binary;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Genre {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Binary>,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GenreDTO {
    pub id: Option<Uuid>,
    pub name: String
}

impl GenreDTO {
    pub fn new(id: Option<Uuid>, name: String) -> Self {
        Self { id, name }
    }
}