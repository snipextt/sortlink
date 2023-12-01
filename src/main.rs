use axum::{
    routing::{get, post},
    Router,
};
use rbatis::RBatis;
use rbdc_pg::PgDriver;
use route::{create_shortlink, redirect_shortlink};
use shuttle_runtime::CustomError;
use shuttle_secrets::SecretStore;
use state::AppState;

mod route;
mod schema;
mod state;

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let pg_uri = store.get("PG_URI").expect("Postgress connection not set");
    let rb = RBatis::new();
    rb.link(PgDriver {}, pg_uri.as_str())
        .await
        .map_err(CustomError::new)?;
    let router = Router::new()
        .route("/shortlink", post(create_shortlink))
        .route("/:shortlink", get(redirect_shortlink))
        .with_state(AppState::new(rb));

    Ok(router.into())
}
