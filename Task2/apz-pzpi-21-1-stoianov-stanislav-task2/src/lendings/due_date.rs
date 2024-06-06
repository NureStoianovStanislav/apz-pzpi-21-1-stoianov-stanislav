use chrono::{Days, NaiveDate};
use serde::Serialize;

use super::lending_date::LendingDate;

pub type LentFor = u64;

#[derive(Clone, Debug, Default, sqlx::Type, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct DueDate(NaiveDate);

impl DueDate {
    pub fn new(lending_date: LendingDate, lent_for: LentFor) -> Self {
        Self(NaiveDate::from(lending_date) + Days::new(lent_for))
    }

    pub fn lent_for(&self, lending_date: LendingDate) -> i64 {
        (self.0 - NaiveDate::from(lending_date)).num_days()
    }
}
