use std::sync::Arc;

use aes::{cipher::KeyInit, Aes128};

use crate::{
    config::{AppConfig, BackupConfig, HasherConfig, JwtConfig},
    database::{self, Database},
};

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub id_cipher: Arc<Aes128>,
    pub jwt_config: Arc<JwtConfig>,
    pub hasher_config: Arc<HasherConfig>,
    pub backup_config: Arc<BackupConfig>,
}

impl AppState {
    pub fn init(config: AppConfig) -> Self {
        Self {
            database: database::connect(config.database),
            id_cipher: Arc::new(Aes128::new(&config.id_key.into())),
            jwt_config: Arc::new(config.jwt),
            hasher_config: Arc::new(config.hasher),
            backup_config: Arc::new(config.backup),
        }
    }
}
