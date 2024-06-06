use std::{path::PathBuf, str::FromStr, time::Duration};

use anyhow::Context;
use argon2::Params;
use secrecy::Secret;
use serde::{Deserialize, Deserializer};
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_with::{serde_as, Bytes, DurationSeconds};
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

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    #[serde_as(as = "Bytes")]
    pub id_key: [u8; 16],
    pub jwt: JwtConfig,
    pub hasher: HasherConfig,
    pub backup: BackupConfig,
}

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
    pub key: Secret<String>,
    #[serde(flatten, deserialize_with = "deserialize_argon2_params")]
    pub params: Params,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct JwtConfig {
    pub key: Secret<String>,
    #[serde_as(as = "DurationSeconds<u64>")]
    pub access_ttl: Duration,
    #[serde_as(as = "DurationSeconds<u64>")]
    pub refresh_ttl: Duration,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BackupConfig {
    pub cmd: String, 
    pub args: Vec<String>,
}

impl Config {
    pub fn init() -> anyhow::Result<Config> {
        config::Config::builder()
            .add_source(config::File::from(config_path(environment()?)?))
            .add_source(config::Environment::default().separator("__"))
            .build()
            .context("read config")?
            .try_deserialize()
            .context("parse config")
    }
}

fn environment() -> anyhow::Result<Environment> {
    std::env::var("ENVIRONMENT")
        .context("ENVIRONMENT must be present")
        .map(|env| Environment::from_str(env.as_str()))?
        .with_context(|| {
            format!("environment must be one of: {:?}", Environment::VARIANTS)
        })
}

fn config_path(environment: Environment) -> anyhow::Result<PathBuf> {
    std::env::current_dir()
        .context("read current working directory")
        .map(|dir| dir.join("config").join(format!("{environment}.yaml")))
        .context("read config file")
}

#[derive(Deserialize)]
struct Argon2Params {
    pub memory_size: u32,
    pub iterations: u32,
    pub parallelism_factor: u32,
    pub output_length: Option<usize>,
}

fn deserialize_argon2_params<'de, D>(
    deserializer: D,
) -> Result<Params, D::Error>
where
    D: Deserializer<'de>,
{
    let params = Argon2Params::deserialize(deserializer)?;
    Params::new(
        params.memory_size,
        params.iterations,
        params.parallelism_factor,
        params.output_length,
    )
    .map_err(serde::de::Error::custom)
}
