use anyhow::Context;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

pub fn init() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .pretty()
        .init()
}

// TODO doesn't enter span for some reason
pub(crate) async fn instrument_blocking<F, R>(f: F) -> anyhow::Result<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    tokio::task::spawn_blocking(|| tracing::Span::current().in_scope(f))
        .await
        .context("spawn blocking task")
}

pub fn debug(value: &impl std::fmt::Debug) {
    tracing::debug!("{value:?}")
}

pub fn error(value: &impl std::fmt::Debug) {
    tracing::error!("{value:?}")
}
