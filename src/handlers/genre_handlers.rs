use axum::extract::{State};
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

pub async fn genre_get(state: State<AppState>) -> impl IntoResponse {
    match state.genre_service.get_genres().await {
        Ok(genres) => Json(genres).into_response(),
        Err((status, message)) => {
            let body = Json(ErrorResponse { message });
            (status, body).into_response()
        }
    }
}