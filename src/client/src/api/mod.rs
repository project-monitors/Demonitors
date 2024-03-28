mod router;
mod controller;
mod response;
mod middleware;

use crate::core::conf::ClientConfig;
use crate::data::mem_db::Conn;
use tokio::net::TcpListener;
use tracing::*;



#[derive(Clone)]
pub struct AppState {
    pub cfg: ClientConfig,
    pub db: Conn
}

async fn app_state(cfg: ClientConfig, db: Conn) -> AppState {
    AppState {
        cfg,
        db
    }
}

pub async fn serve(cfg: ClientConfig, db: Conn) {
    let port = cfg.api.port;
    let state = app_state(cfg, db).await;

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await.unwrap();

    info!("listening on {}", port);

    axum::serve(listener, router::init(state))
        .await
        .unwrap();

}