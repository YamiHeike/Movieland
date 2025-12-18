use serde::{Serialize, Deserialize };
use mongodb::bson::{Binary, uuid::{Uuid as MongoUuid, UuidRepresentation}};
use uuid::Uuid;
use crate::errors::GenreError;

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

    pub fn to_dto(&self) -> Result<GenreDTO, GenreError> {
        let id = match &self.id {
            Some(bin_id) => Uuid::from_slice(&bin_id.bytes).map_err(|_| GenreError::InvalidId)?,
            None => Uuid::new_v4()
        };
        Ok(GenreDTO::new(Some(id), String::from(&self.name)))
    }
}

impl GenreDTO {
    pub fn new(id: Option<Uuid>, name: String) -> Self {
        Self { id, name }
    }

    pub fn to_genre(&self) -> Result<Genre, GenreError> {
        let uuid = match self.id {
            Some(standard_uuid) => {
                let mongo_uuid = MongoUuid::from_bytes(*standard_uuid.as_bytes());
                Binary::from_uuid_with_representation(mongo_uuid, UuidRepresentation::Standard)
            }
            None => Binary::from_uuid_with_representation(MongoUuid::new(), UuidRepresentation::Standard),
        };
        Ok(Genre::new(Some(uuid), self.name.clone()))
    }

}