use axum::extract::{State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::models::genre::{GenreDTO};

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

pub async fn genre_get(state: State<AppState>) -> impl IntoResponse {
    match state.genre_service.get_genres().await {
        Ok(genres) => Json(genres).into_response(),
        Err(err) => err.into_response()
    }
}

pub async fn genre_post(state: State<AppState>, genre: Json<GenreDTO>) -> impl IntoResponse {
    match state.genre_service.post_genres(genre).await {
        Ok(genre) => (StatusCode::CREATED, Json(genre).into_response()),
        Err(err) => {
            let res = err.into_response();
            (res.status(), res)
        }
    }
}