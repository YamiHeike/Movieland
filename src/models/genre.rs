use serde::{Serialize, Deserialize };
use mongodb::bson::Binary;
use mongodb::bson::{uuid::{Uuid as MongoUuid, UuidRepresentation}};
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

impl Genre {
    pub fn new(id: Option<Binary>, name: String) -> Self {
        Self { id, name }
    }

    pub fn to_dto(&self) -> GenreDTO {
        let id = self.id.as_ref()
            .map(|bin_id| Uuid::from_slice(&bin_id.bytes).unwrap())
            .unwrap_or_else(Uuid::new_v4);
        GenreDTO::new(Some(id), String::from(&self.name))
    }
}

impl GenreDTO {
    pub fn new(id: Option<Uuid>, name: String) -> Self {
        Self { id, name }
    }

    pub fn to_genre(&self) -> Genre {
        let uuid = Binary::from_uuid_with_representation(MongoUuid::new(), UuidRepresentation::Standard);
        Genre::new(Some(uuid), String::from(&self.name))
    }
}