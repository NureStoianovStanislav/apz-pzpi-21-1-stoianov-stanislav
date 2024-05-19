use crate::{
    config::AppConfig,
    database::{self, Database},
};

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
}

impl AppState {
    pub fn init(config: AppConfig) -> Self {
        let database = database::connect(config.database);
        Self { database }
    }
}
