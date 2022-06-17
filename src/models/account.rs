use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    #[serde(skip)]
    pub code: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct AccountHibernate {
    pub code: String,
    pub description: String,
}

impl From<&Account> for AccountHibernate {
    fn from(model: &Account) -> Self {
        Self {
            code: model.code.clone(),
            description: model.description.clone(),
        }
    }
}
