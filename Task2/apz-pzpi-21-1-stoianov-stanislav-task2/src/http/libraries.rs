use axum::{extract::State, http::StatusCode, routing::post, Form, Router};

use crate::{auth::UserId, libraries::add_library, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/",
        post(|admin_id: UserId, State(state), Form(library)| async move {
            add_library(admin_id, library, state)
                .await
                .map(|_| StatusCode::CREATED)
        }),
    )
}
