use axum::http::StatusCode;
use axum::Json;
use mongodb::{Collection, Database};
use mongodb::bson::{doc, uuid::{Uuid as MongoUuid, UuidRepresentation}, Binary};
use mongodb::error::Error;
use uuid::Uuid;
use crate::models::genre::{Genre, GenreDTO};

#[derive(Clone)]
pub struct GenreService {
    db: Database,
}

impl GenreService {
    pub fn new(db: Database) -> GenreService {
        GenreService { db }
    }

    fn get_genres_collection(&self) -> Collection<Genre> {
        self.db.collection::<Genre>("genres")
    }

    pub async fn get_genres(&self) -> Result<Vec<GenreDTO>, (StatusCode, String)> {
        let col = self.get_genres_collection();
        let mut cursor = col.find(doc! {})
            .await
            .map_err(Self::err)?;

        let mut genres = Vec::new();
        while cursor.advance().await.map_err(Self::err)? {
            let genre_entity = cursor.deserialize_current()
                .map_err(Self::err)?;
            let id = genre_entity.id.map(|b| Uuid::from_slice(&b.bytes).unwrap());
            genres.push(GenreDTO::new(id, genre_entity.name));
        }
        Ok(genres)
    }

    pub async fn post_genres(&self, genre: Json<GenreDTO>) -> Result<GenreDTO, (StatusCode, String)> {
        let genre_collection = self.get_genres_collection();
        let uuid = Binary::from_uuid_with_representation(MongoUuid::new(), UuidRepresentation::Standard);
        let id = Some(Uuid::from_slice(&uuid.bytes).unwrap());
        let genre_entity = Genre {
            id: Some(uuid),
            name: String::from(&genre.name),
        };
        let genre_dto = GenreDTO {
            id,
            name: String::from(&genre.name),
        };

        genre_collection.insert_one(&genre_entity).await.map_err(Self::err)?;
        Ok(genre_dto)
    }
    fn err(e: Error) -> (StatusCode, String) {
        (StatusCode::BAD_REQUEST, e.to_string())
    }

}