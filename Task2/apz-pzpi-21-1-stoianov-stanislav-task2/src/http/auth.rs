use anyhow::Context;
use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Form, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::{
    auth::{parse_access_token, sign_in, sign_up, TokenPair, UserId},
    state::AppState,
    Error,
};

static ACCESS_TOKEN: &str = "access-token";
static REFRESH_TOKEN: &str = "refresh-token";

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/sign-up",
            post(|State(state), Form(credentials)| async {
                sign_up(credentials, state)
                    .await
                    .map(|_| StatusCode::CREATED)
            }),
        )
        .route(
            "/sign-in",
            post(|State(state), Form(credentials)| sign_in(credentials, state)),
        )
        .route(
            "/me",
            axum::routing::get(|id: UserId, State(state): State<AppState>| async move {
                axum::Json(id.sql_id(&state.id_cipher).map_err(|e| e.to_string()))
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
        parse_access_token(cookie.value(), &state.jwt_config).map_err(|_| Error::LoggedOff)
    }
}

impl IntoResponse for TokenPair {
    fn into_response(self) -> Response {
        let access = {
            let mut cookie = Cookie::new(ACCESS_TOKEN, self.access_token);
            cookie.set_path("/");
            cookie.set_http_only(true);
            cookie
        };
        let refresh = {
            let mut cookie = Cookie::new(REFRESH_TOKEN, self.refresh_token);
            cookie.set_path("/auth/refresh");
            cookie.set_http_only(true);
            cookie
        };
        CookieJar::new().add(access).add(refresh).into_response()
    }
}
