use anyhow::Context;

use crate::{
    auth::{check_permission, Role, UserId},
    state::AppState,
};

#[tracing::instrument(skip(state), err(Debug))]
pub async fn backup(
    admin_id: UserId,
    state: AppState,
) -> crate::Result<Vec<u8>> {
    check_permission(admin_id, &state, |role| {
        matches!(role, Role::Administrator)
    })
    .await?;
    let args = state.backup_config.args.iter().collect::<Vec<_>>();
    std::process::Command::new(state.backup_config.cmd.as_str())
        .args(args.as_slice())
        .output()
        .map(|out| out.stdout)
        .context("execute pg_dump")
        .map_err(crate::Error::from)
}
