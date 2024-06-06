use axum::{extract::State, routing::get, Router};

use crate::{auth::UserId, backup::backup, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/",
        get(|admin_id: UserId, State(state)| async move {
            backup(admin_id, state).await
        }),
    )
}
