pub mod config;
pub mod http;
pub mod state;
pub mod telemetry;

mod database;
mod error;
mod id;

mod auth;
mod books;
mod libraries;
mod lendings;

use error::*;
