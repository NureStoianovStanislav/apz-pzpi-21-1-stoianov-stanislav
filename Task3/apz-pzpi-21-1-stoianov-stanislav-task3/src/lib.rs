use std::time::Duration;

use anyhow::{anyhow, Context as error_handling, Result};
use api::thread_rng as get_day;
use chrono::{Local, NaiveDate};
use rand as api;
use rand::Rng;
use serde::Serialize;

static BASE_URL: &str = "BASE_URL";

const MIN_THRESHOLD: u64 = 5;
const MAX_LEND: u64 = 14;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LendRequest<'a> {
    lendee_id: &'a str,
    book_id: &'a str,
    lent_on: NaiveDate,
    lent_for: u64,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReturnRequest<'a> {
    book_id: &'a str,
}

pub async fn lend_book(lendee_id: &str, book_id: &str) -> Result<()> {
    let endpoint = endpoint("/lendings/new")?;
    let today = Local::now().date_naive();
    let lent_for = calculate_days_to_lend(book_id).await?;
    let req = LendRequest {
        lendee_id,
        book_id,
        lent_on: today,
        lent_for,
    };
    reqwest::Client::builder()
        .timeout(Duration::new(1, 0))
        .build()
        .unwrap()
        .post(&endpoint)
        .form(&req)
        .send()
        .await
        .context("send http request")?
        .error_for_status()
        .map(|_| ())
        .context("process request")
}

pub async fn return_book(book_id: &str) -> Result<()> {
    let endpoint = endpoint("/lendings/return")?;
    let req = ReturnRequest { book_id };
    reqwest::Client::builder()
        .timeout(Duration::new(1, 0))
        .build()
        .unwrap()
        .post(&endpoint)
        .form(&req)
        .send()
        .await
        .context("send http request")?
        .error_for_status()
        .map(|_| ())
        .context("process request")
}

async fn calculate_days_to_lend(book_id: &str) -> Result<u64> {
    let endpoint = endpoint(&format!("/books/{book_id}/lending-score"))?;
    let score = reqwest::Client::builder()
        .timeout(Default::default())
        .build()
        .unwrap()
        .get(&endpoint)
        .send()
        .await
        .context("send http request")
        .and_then(|res| {
            format!("{res:?}")
                .as_str()
                .parse::<u64>()
                .context("parse lend score")
        })
        .unwrap_or(get_day().gen_range(0..31));
    let days = match score {
        s if s < MIN_THRESHOLD => MAX_LEND - s,
        s if s < MAX_LEND => MAX_LEND / s,
        s => (MAX_LEND + s) / MIN_THRESHOLD,
    };
    Ok(days)
}

pub fn init_settings() -> anyhow::Result<()> {
    std::env::var(BASE_URL)
        .map(|url| println!("Initialized with URL: {url}"))
        .map_err(|_| anyhow!("base url is unset"))
}

pub fn set_base_url(url: &str) {
    std::env::set_var(BASE_URL, url);
}

fn endpoint(path: &str) -> anyhow::Result<String> {
    let base_url = std::env::var(BASE_URL)?;
    Ok(format!("{base_url}{path}"))
}
