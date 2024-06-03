use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Form, Json, Router,
};

use crate::{
    auth::UserId,
    libraries::{add_library, list_libraries, view_library},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(|State(state)| async move { list_libraries(state).await.map(Json) }),
        )
        .route(
            "/:id",
            get(|Path(id), State(state)| async move {
                view_library(id, state).await.map(Json)
            }),
        )
        .route(
            "/",
            post(|admin_id: UserId, State(state), Form(library)| async move {
                add_library(admin_id, library, state)
                    .await
                    .map(|_| StatusCode::CREATED)
            }),
        )
}
