use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::Redirect,
};

use crate::{schema::Shortlink, state::AppState};

pub async fn redirect_shortlink(
    Path(shortlink): Path<String>,
    State(state): State<AppState>,
) -> Result<Redirect, StatusCode> {
    match Shortlink::find_shortlinks(&state.db, shortlink.as_str()).await {
        Ok(row) => {
            if row.is_none() {
                Err(StatusCode::NOT_FOUND)
            } else {
                Ok(Redirect::temporary(row.unwrap().link.as_str()))
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_shortlink(
    State(state): State<AppState>,
    Json(shortlink): Json<Shortlink>,
) -> StatusCode {
    match Shortlink::insert(&state.db, &shortlink).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
