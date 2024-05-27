use crate::{
    config::{AppConfig, HasherConfig},
    database::{self, Database},
};

#[derive(Clone)]
pub struct AppState {
    pub hasher: HasherConfig,
    pub database: Database,
}

impl AppState {
    pub fn init(config: AppConfig) -> Self {
        let database = database::connect(config.database);
        Self {
            hasher: config.hasher,
            database,
        }
    }
}
