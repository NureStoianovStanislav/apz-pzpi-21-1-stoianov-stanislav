use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Form, Json, Router,
};

use crate::{
    auth::UserId,
    lendings::{active_lendings, lend_book, return_book},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/new",
            post(|State(state), Form(lending)| async move {
                lend_book(lending, state).await.map(|_| StatusCode::CREATED)
            }),
        )
        .route(
            "/:library_id/pending",
            get(
                |owner_id: UserId, Path(library_id), State(state)| async move {
                    active_lendings(owner_id, library_id, state).await.map(Json)
                },
            ),
        )
        .route(
            "/return",
            post(|State(state), Form(return_request)| async move {
                return_book(return_request, state).await
            }),
        )
}
