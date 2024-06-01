use std::net::SocketAddr;

use anyhow::Context;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Form, Json, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use secrecy::ExposeSecret;
use tokio::net::TcpListener;

use crate::{
    auth::{sign_in, sign_up, TokenPair},
    config::HttpConfig,
    state::AppState,
};

static ACCESS_TOKEN: &str = "access-token";
static REFRESH_TOKEN: &str = "refresh-token";

pub async fn serve(config: HttpConfig, state: AppState) -> anyhow::Result<()> {
    let router = root().with_state(state);
    let addr = SocketAddr::from((config.host, config.port));
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router.into_make_service())
        .await
        .context("start http server")
}

fn root() -> Router<AppState> {
    Router::new().nest("/auth", auth())
}

fn auth() -> Router<AppState> {
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
}

#[derive(serde::Serialize)]
struct ErrorMessage {
    error: String,
}

impl IntoResponse for crate::Error {
    fn into_response(self) -> Response {
        use crate::Error;
        let code = match self {
            Error::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Error::AccountExists => StatusCode::CONFLICT,
            Error::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = ErrorMessage {
            error: self.to_string(),
        };
        (code, Json(message)).into_response()
    }
}

impl IntoResponse for TokenPair {
    fn into_response(self) -> Response {
        let access = {
            let mut cookie =
                Cookie::new(ACCESS_TOKEN, self.access_token.expose_secret().to_string());
            cookie.set_path("/");
            cookie.set_http_only(true);
            cookie
        };
        let refresh = {
            let mut cookie = Cookie::new(
                REFRESH_TOKEN,
                self.refresh_token.expose_secret().to_string(),
            );
            cookie.set_path("/auth/refresh");
            cookie.set_http_only(true);
            cookie
        };
        CookieJar::new().add(access).add(refresh).into_response()
    }
}
