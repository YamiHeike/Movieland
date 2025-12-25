use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenreError {
    #[error("genre not found")]
    NotFound,
    #[error("genre already exists")]
    Duplicate,
    #[error("invalid genre id")]
    InvalidId,
    #[error("database error: {0}")]
    Database(#[from] mongodb::error::Error),
}

impl IntoResponse for GenreError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            GenreError::NotFound => (StatusCode::NOT_FOUND, String::from("Genre Not Found")),
            GenreError::Duplicate => (StatusCode::BAD_REQUEST, String::from("Genre Already Exists")),
            GenreError::InvalidId => (StatusCode::BAD_REQUEST, String::from("Invalid Genre Id")),
            GenreError::Database(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal server error: {err}")),
        };
        (status, message).into_response()
    }
}