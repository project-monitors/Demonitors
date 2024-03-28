use super::AppState;
use super::middleware::cors_handle;
use super::controller::*;
use axum::{
    routing::{get},
    Router,
};

pub fn init(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/oracle_data/:timestamp", get(get_oracle_data))
        .layer(axum::middleware::from_fn(cors_handle))
        .with_state(state)
}