use axum::http::StatusCode;
use axum::Json;
use mongodb::{Collection, Database};
use mongodb::bson::{doc, uuid::{Uuid as MongoUuid, UuidRepresentation}, Binary};
use mongodb::error::Error;
use uuid::Uuid;
use crate::errors::GenreError;
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

    pub async fn get_genres(&self) -> Result<Vec<GenreDTO>, GenreError> {
        let col = self.get_genres_collection();
        let mut cursor = col.find(doc! {})
            .await?;
        let mut genres = Vec::new();

        while cursor.advance().await? {
            let genre_entity = cursor.deserialize_current()?;
            let id = genre_entity.id
                .map(|b| Uuid::from_slice(&b.bytes).map_err(|_| GenreError::InvalidId).unwrap());
            genres.push(GenreDTO::new(id, genre_entity.name));
        }
        Ok(genres)
    }

    pub async fn post_genres(&self, genre_dto: Json<GenreDTO>) -> Result<GenreDTO, GenreError> {
        let genre_collection = self.get_genres_collection();
        let genre_entity = genre_dto.to_genre()?;
        let genre_dto = genre_entity.to_dto()?;

        genre_collection.insert_one(&genre_entity).await?;
        Ok(genre_dto)
    }

}