use axum::extract::{State};
use axum::{Json, http::StatusCode, response::IntoResponse, extract::Path};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
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

pub async fn genre_get_by_id(state: State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match state.genre_service.get_genre_by_id(id).await {
        Ok(genre) => (StatusCode::OK, Json(genre)).into_response(),
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

pub async fn genre_patch(state: State<AppState>, genre: Json<GenreDTO>) -> impl IntoResponse {
    match state.genre_service.patch_genre(genre).await {
        Ok(genre) => (StatusCode::OK, Json(genre)).into_response(),
        Err(err) => err.into_response()
    }
}