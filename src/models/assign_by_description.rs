use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AssignByDescription {
    pub account_code: String,
    pub search_expression: Regex,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct AssignByDescriptionDefinition {
    #[serde(rename = "account")]
    pub account_code: String,
    #[serde(rename = "search")]
    pub search_expression: String,
    pub note: Option<String>,
}

impl TryFrom<&AssignByDescriptionDefinition> for AssignByDescription {
    type Error = regex::Error;

    fn try_from(value: &AssignByDescriptionDefinition) -> core::result::Result<Self, Self::Error> {
        let search_expression = Regex::new(format!("(?i){}", value.search_expression).as_str())?;
        Ok(Self {
            account_code: value.account_code.clone(),
            search_expression,
        })
    }
}
