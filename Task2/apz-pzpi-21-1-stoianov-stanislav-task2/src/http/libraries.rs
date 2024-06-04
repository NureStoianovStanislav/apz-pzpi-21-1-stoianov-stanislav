use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Form, Json, Router,
};

use crate::{
    auth::UserId,
    books::{
        add_book, delete_book, list_library_books, update_book, view_book,
    },
    libraries::{
        add_library, delete_library, list_libraries, list_my_libraries,
        update_library, view_library,
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/:id/books", books_router())
        .route(
            "/",
            get(|State(state)| async move { list_libraries(state).await.map(Json) }),
        )
        .route(
            "/my",
            get(|user_id: UserId, State(state)| async move { list_my_libraries(user_id, state).await.map(Json) }),
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

fn books_router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(
                |owner_id: UserId,
                 Path(library_id),
                 State(state),
                 Form(book)| async move {
                    add_book(owner_id, library_id, book, state)
                        .await
                        .map(|_| StatusCode::CREATED)
                },
            ),
        )
        .route(
            "/",
            get(|Path(library_id), State(state)| async move {
                list_library_books(library_id, state).await.map(Json)
            }),
        )
        .route(
            "/:id",
            get(|Path((library_id, book_id)), State(state)| async move {
                view_book(library_id, book_id, state).await.map(Json)
            }),
        )
        .route(
            "/:id",
            put(
                |owner_id: UserId,
                 Path((library_id, book_id)),
                 State(state),
                 Form(book)| async move {
                    update_book(owner_id, library_id, book_id, book, state)
                        .await
                },
            ),
        )
        .route(
            "/:id",
            delete(
                |owner_id: UserId,
                 Path((library_id, book_id)),
                 State(state)| async move {
                    delete_book(owner_id, library_id, book_id, state).await
                },
            ),
        )
}
