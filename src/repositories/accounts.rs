use serde::{Deserialize, Serialize};
use serde_json::{self, value::Value as JsonValue};
use std::{collections::HashMap, fmt::Display};

use crate::Account;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Accounts {
    records: HashMap<String, Account>,
}

impl Accounts {
    pub fn load<S>(src: S) -> Result<Accounts, std::io::Error>
    where
        S: std::io::Read,
    {
        let import: Vec<Account> = serde_json::from_reader(src)?;
        let records = import
            .into_iter()
            .fold(HashMap::new(), |mut records, account| {
                records.insert(account.code.clone(), account);

                records
            });

        Ok(Accounts { records })
    }

    pub fn as_json(&self) -> JsonValue {
        serde_json::from_str(self.to_string().as_ref()).unwrap()
    }
}

impl Display for Accounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = if f.alternate() {
            serde_json::to_string_pretty(&self.records).unwrap()
        } else {
            serde_json::to_string(&self.records).unwrap()
        };
        f.write_str(str.as_ref())
    }
}
