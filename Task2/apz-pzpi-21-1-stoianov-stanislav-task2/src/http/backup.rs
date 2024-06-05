use anyhow::Context;
use axum::{extract::State, routing::get, Router};

use crate::{
    auth::{check_permission, Role, UserId},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/",
        get(|admin_id: UserId, State(state)| async move {
            check_permission(admin_id, &state, |role| {
                matches!(role, Role::Administrator)
            })
            .await?;
            backup()
        }),
    )
}

#[tracing::instrument(err(Debug))]
fn backup() -> crate::Result<Vec<u8>> {
    std::process::Command::new("docker")
        .args(["compose", "exec", "postgres", "pg_dump"])
        .output()
        .map(|out| out.stdout)
        .context("execute pg_dump")
        .map_err(crate::Error::from)
}
