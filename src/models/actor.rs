use serde::{ Serialize, Deserialize };
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Actor {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub age: u8
}