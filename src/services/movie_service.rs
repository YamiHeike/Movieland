use mongodb::{Database, bson::{doc, to_bson}, error::Result as MongoResult};
use mongodb::{Collection};
use uuid::Uuid;
use crate::entities::movie::Movie;

#[derive(Clone)]
pub struct MovieService {
    db: Database
}

impl MovieService {
    pub fn new(db: Database) -> MovieService {
        MovieService{ db }
    }

    fn get_movie_collection(&self) -> Collection<Movie> {
        self.db.collection::<Movie>("movies")
    }

    pub async fn create_movie(&self, mut movie: Movie) -> MongoResult<Movie> {
        let movie_id = Some(Uuid::new_v4());
        movie.id = movie_id;
        self.get_movie_collection()
            .insert_one(&movie)
            .await?;
        Ok(movie)
    }
}


