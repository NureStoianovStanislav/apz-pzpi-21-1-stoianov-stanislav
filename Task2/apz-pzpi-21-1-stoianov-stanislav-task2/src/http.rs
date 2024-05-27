use std::net::SocketAddr;

use anyhow::Context;
use axum::{extract::State, routing::post, Json, Router};
use tokio::net::TcpListener;

use crate::{auth::sign_up, config::HttpConfig, state::AppState};

pub async fn serve(config: HttpConfig, state: AppState) -> anyhow::Result<()> {
    let router = root().with_state(state);
    let addr = SocketAddr::from((config.host, config.port));
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router.into_make_service())
        .await
        .context("start http server")
}

fn root() -> Router<AppState> {
    Router::new().nest("/auth", auth())
}

fn auth() -> Router<AppState> {
    Router::new().route(
        "/sign-up",
        post(|State(state), Json(credentials)| sign_up(credentials, state)),
    )
}
