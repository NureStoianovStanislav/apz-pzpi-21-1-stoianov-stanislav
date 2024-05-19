use libmarse::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    libmarse::telemetry::init();
    let config = libmarse::config::Config::init()?;
    let state = AppState::init(config.app);
    libmarse::http::serve(config.http, state).await
}
