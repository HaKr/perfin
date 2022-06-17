use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignByContractDefinition {
    #[serde(rename = "account")]
    pub account_code: String,
    pub description: Option<String>,
    pub note: Option<String>,
}
