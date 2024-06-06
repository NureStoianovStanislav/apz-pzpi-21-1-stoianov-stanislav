use anyhow::Context;
use axum::{extract::State, routing::get, Router};

use crate::{
    auth::{check_permission, Role, UserId},
    config::BackupConfig,
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
            backup(&state.backup_config)
        }),
    )
}

#[tracing::instrument(err(Debug))]
fn backup(config: &BackupConfig) -> crate::Result<Vec<u8>> {
    let args = config.args.iter().collect::<Vec<_>>();
    std::process::Command::new(config.cmd.as_str())
        .args(args.as_slice())
        .output()
        .map(|out| out.stdout)
        .context("execute pg_dump")
        .map_err(crate::Error::from)
}
