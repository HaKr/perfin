use std::collections::HashMap;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AssignmentReason {
    Reference,
    RelationName,
    Contract,
    Description,
}

#[derive(Serialize)]
#[serde(rename = "camelCase")]
pub struct BankTransaction {
    pub id: String,
    pub date: NaiveDate,

    pub cost_center: String,
    pub relation_name: Option<String>,
    pub relation_iban: Option<String>,

    pub attributes: HashMap<String, String>,
    pub amount: Decimal,

    pub account_code: Option<String>,
    pub assignment_reason: Option<AssignmentReason>,
}
