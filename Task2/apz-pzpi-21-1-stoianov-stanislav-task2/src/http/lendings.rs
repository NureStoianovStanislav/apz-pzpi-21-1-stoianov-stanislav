use axum::{extract::State, http::StatusCode, routing::post, Form, Router};

use crate::{lendings::lend_book, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/new",
        post(|State(state), Form(lending)| async move {
            lend_book(lending, state).await.map(|_| StatusCode::CREATED)
        }),
    )
}
