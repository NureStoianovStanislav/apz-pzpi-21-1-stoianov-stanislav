use std::net::SocketAddr;

use anyhow::Context;
use axum::{Json, Router};
use tokio::net::TcpListener;

use crate::{
    auth::{self, User},
    config::HttpConfig,
    state::AppState,
};

pub async fn serve(config: HttpConfig, state: AppState) -> anyhow::Result<()> {
    let router = Router::new().with_state(state);
    let addr = SocketAddr::from((config.host, config.port));
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router.into_make_service())
        .await
        .context("failed to start http server")
}

pub async fn do_stuff(Json(user): Json<User>) -> crate::Result<Json<User>> {
    auth::do_stuff(&user).await.map(Json)
}
