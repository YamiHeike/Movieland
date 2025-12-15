mod db_config;

use axum::{serve, Router};
use axum::routing::{get};
use mongodb::Database;
use crate::db_config::DbConfig;
use crate::handlers::genre_handlers::{genre_get, genre_post};
use crate::services::genre_service::GenreService;

mod services;
mod models;
mod common;
mod handlers;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub genre_service: GenreService,
}

#[tokio::main]
async fn main() {
    let db = DbConfig::create_db().await;
    let app = create_app(db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Could not bind server");
    serve(listener, app).await.expect("Could not start server");
}

fn create_app(db: Database) -> Router {

    let genre_service = GenreService::new(db.clone());

    let state = AppState {
        db, genre_service
    };

    let app = Router::new()
        .route("/genres", get(genre_get).post(genre_post))
        .with_state(state);
    app
}