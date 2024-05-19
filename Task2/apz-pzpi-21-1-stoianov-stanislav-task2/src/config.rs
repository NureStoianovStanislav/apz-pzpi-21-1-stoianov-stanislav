use std::{path::PathBuf, str::FromStr};

use anyhow::Context;
use secrecy::Secret;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_with::serde_as;
use strum::VariantNames;
use strum_macros::{Display, EnumString, VariantNames};

#[derive(Clone, Copy, Debug, Display, EnumString, VariantNames)]
#[strum(serialize_all = "lowercase")]
enum Environment {
    Development,
    Production,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub http: HttpConfig,
    #[serde(flatten)]
    pub app: AppConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub hasher: HasherConfig,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct HttpConfig {
    pub host: [u8; 4],
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: Secret<String>,
    pub database: String,
    pub require_ssl: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HasherConfig {
    pub secret: Secret<String>,
    pub memory_size: u32,
    pub iterations: u32,
    pub parallelism_factor: u32,
    pub output_length: Option<usize>,
}

impl Config {
    pub fn init() -> anyhow::Result<Config> {
        config::Config::builder()
            .add_source(config::File::from(config_path(environment()?)?))
            .add_source(config::Environment::default().separator("__"))
            .build()
            .context("failed to read config")?
            .try_deserialize()
            .context("failed to parse config")
    }
}

fn environment() -> anyhow::Result<Environment> {
    std::env::var("ENVIRONMENT")
        .context("ENVIRONMENT must be present")
        .map(|env| Environment::from_str(env.as_str()))?
        .with_context(|| format!("environment must be one of: {:?}", Environment::VARIANTS))
}

fn config_path(environment: Environment) -> anyhow::Result<PathBuf> {
    std::env::current_dir()
        .context("failed to read current working directory")
        .map(|dir| dir.join("config").join(format!("{environment}.yaml")))
        .context("failed to read config file")
}
