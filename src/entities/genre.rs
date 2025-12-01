use serde::{ Serialize, Deserialize };
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Genre {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub name: String,
}