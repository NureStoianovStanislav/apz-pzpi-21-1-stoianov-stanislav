mod auth;
mod error;
mod libraries;

use std::net::SocketAddr;

use anyhow::Context;
use axum::Router;
use tokio::net::TcpListener;

use crate::{config::HttpConfig, state::AppState};

pub async fn serve(config: HttpConfig, state: AppState) -> anyhow::Result<()> {
    let router = router().with_state(state);
    let addr = SocketAddr::from((config.host, config.port));
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router.into_make_service())
        .await
        .context("start http server")
}

fn router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/libraries", libraries::router())
}
