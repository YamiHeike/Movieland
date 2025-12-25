use axum::Json;
use mongodb::{Collection, Database};
use mongodb::bson::{doc, Binary, spec::BinarySubtype};
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

    pub async fn get_genre_by_id(&self, id: Uuid) -> Result<GenreDTO, GenreError> {
        let col = self.get_genres_collection();
        let genre = col.find_one(doc! {"_id": Binary {
            subtype: BinarySubtype::UuidOld,
            bytes: id.as_bytes().to_vec(),
        }}).await?;
        if let Some(genre) = genre {
            Ok(genre.to_dto()?)
        } else {
            Err(GenreError::NotFound)
        }
    }

    pub async fn post_genres(&self, genre_dto: Json<GenreDTO>) -> Result<GenreDTO, GenreError> {
        let genre_collection = self.get_genres_collection();

        if let Some(_) = self.get_genre_by_name(&genre_dto.name).await {
            return Err(GenreError::Duplicate);
        }

        let genre_entity = genre_dto.to_genre()?;
        let genre_dto = genre_entity.to_dto()?;

        genre_collection.insert_one(&genre_entity).await?;
        Ok(genre_dto)
    }

    pub async fn patch_genre(&self, genre_dto: Json<GenreDTO>) -> Result<GenreDTO, GenreError> {
        let col = self.get_genres_collection();

        let id = genre_dto.id.ok_or(GenreError::InvalidId)?;

        self.get_genre_by_id(id).await.map_err(|_| GenreError::NotFound)?;

        if let Some(existing) = self.get_genre_by_name(&genre_dto.name).await {
            let existing_id = existing.to_dto()?.id.ok_or(GenreError::InvalidId)?;
            if existing_id != id {
                return Err(GenreError::Duplicate);
            }
        }

        col.update_one(doc! { "_id": Binary {
            subtype: BinarySubtype::UuidOld,
            bytes: id.as_bytes().to_vec(),
        }}, doc! { "$set": { "name": &genre_dto.name }}).await?;

        Ok(GenreDTO::new(genre_dto.id, String::from(&genre_dto.name)))
    }

    async fn get_genre_by_name(&self, name: &String) -> Option<Genre> {
        let col = self.get_genres_collection();
        col.find_one(doc! { "name": name }).await.ok().flatten()
    }

}