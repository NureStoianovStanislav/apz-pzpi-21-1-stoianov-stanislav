use anyhow::Context;
use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Form, Json, Router,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};

use crate::{
    auth::{
        get_all_users, get_user, parse_access_token, sign_in, sign_up,
        update_user, TokenPair, UserId,
    },
    state::AppState,
    Error,
};

static ACCESS_TOKEN: &str = "access-token";
static REFRESH_TOKEN: &str = "refresh-token";

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/sign-up",
            post(|State(state), Form(credentials)| async move {
                sign_up(credentials, state)
                    .await
                    .map(|_| StatusCode::CREATED)
            }),
        )
        .route(
            "/sign-in",
            post(|State(state), Form(credentials)| async move {
                sign_in(credentials, state).await
            }),
        )
        .route(
            "/me",
            get(|id: UserId, State(state)| async move {
                get_user(id, state).await.map(Json)
            }),
        )
        .route(
            "/me",
            put(|id: UserId, State(state), Json(user_info)| async move {
                update_user(id, user_info, state).await
            }),
        )
        .route(
            "/users",
            get(|admin_id: UserId, State(state)| async move {
                get_all_users(admin_id, state).await.map(Json)
            }),
        )
}

#[axum::async_trait]
impl FromRequestParts<AppState> for UserId {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let cookies = CookieJar::from_request_parts(parts, state)
            .await
            .context("extract cookies")?;
        let cookie = cookies.get(ACCESS_TOKEN).ok_or(Error::LoggedOff)?;
        parse_access_token(cookie.value(), &state.jwt_config)
            .map_err(|_| Error::LoggedOff)
    }
}

impl IntoResponse for TokenPair {
    fn into_response(self) -> Response {
        let access = {
            let mut cookie = Cookie::new(ACCESS_TOKEN, self.access_token);
            cookie.set_path("/");
            cookie.set_same_site(SameSite::None);
            cookie.set_http_only(true);
            cookie
        };
        let refresh = {
            let mut cookie = Cookie::new(REFRESH_TOKEN, self.refresh_token);
            cookie.set_path("/auth/refresh");
            cookie.set_same_site(SameSite::None);
            cookie.set_http_only(true);
            cookie
        };
        CookieJar::new().add(access).add(refresh).into_response()
    }
}
