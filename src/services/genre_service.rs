use axum::http::StatusCode;
use mongodb::{Collection, Database};
use mongodb::bson::doc;
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

    fn err(e: Error) -> (StatusCode, String) {
        (StatusCode::BAD_REQUEST, e.to_string())
    }
}