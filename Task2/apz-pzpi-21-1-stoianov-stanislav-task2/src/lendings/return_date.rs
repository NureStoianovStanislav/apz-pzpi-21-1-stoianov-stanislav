use chrono::{Local, NaiveDate};
use serde::Serialize;

#[derive(Clone, Debug, Default, sqlx::Type, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct ReturnDate(NaiveDate);

impl ReturnDate {
    pub fn today() -> Self {
        Self(Local::now().date_naive())
    }
}
