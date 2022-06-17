use chrono::NaiveDate;
use rust_decimal::{serde::float as AsFloat, Decimal};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JournalEntry {
    pub date: NaiveDate,
    pub account_code: String,
    #[serde(with = "AsFloat")]
    pub amount: Decimal,
    pub description: String,
}
