use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Form, Json, Router,
};

use crate::{
    auth::UserId,
    libraries::{
        add_library, delete_library, list_libraries, update_library,
        view_library,
    },
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
        .route(
            "/:id",
            put(|admin_id: UserId, Path(library_id), State(state), Form(library)| async move {
                update_library(admin_id, library_id, library, state)
                    .await
                    .map(|_| StatusCode::OK)
            }),
        )
        .route(
            "/:id",
            delete(|admin_id: UserId, Path(library_id), State(state)| async move {
                delete_library(admin_id, library_id, state)
                    .await
                    .map(|_| StatusCode::OK)
            }),
        )
}
