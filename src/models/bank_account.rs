use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccount {
    #[serde(skip)]
    pub iban: String,
    #[serde(rename = "cost_center")]
    pub cost_center_code: String,
    pub description: String,
}
