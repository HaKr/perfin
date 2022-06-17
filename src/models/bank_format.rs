use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BankFormat {
    pub name: String,
    pub description: String,
}
