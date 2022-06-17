use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    #[serde(skip)]
    pub reference: String,
    pub name: String,
}
