use anyhow::Context;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

pub fn init() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .init()
}

#[allow(unused)]
pub(crate) async fn instrument_blocking<F, R>(f: F) -> anyhow::Result<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    tokio::task::spawn_blocking(|| tracing::Span::current().in_scope(f))
        .await
        .context("failed to spawn blocking task")
}
