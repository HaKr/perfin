use std::{collections::HashMap, fs::File};

use serde::{Deserialize, Serialize};

use crate::BankFormat;

#[derive(Serialize, Deserialize, Default)]
pub struct BankFormats {
    pub formats: HashMap<String, BankFormat>,
}

impl BankFormats {
    pub fn from_fixture() -> Result<Self, std::io::Error> {
        let file = File::open("./data/bank_formats.yaml")?;
        match serde_yaml::from_reader(file) {
            Ok(this) => Ok(this),
            Err(_) => Ok(Self::default()),
        }
    }

    pub fn load<S>(src: S) -> Result<BankFormats, std::io::Error>
    where
        S: std::io::Read,
    {
        let import: Vec<BankFormat> = serde_json::from_reader(src)?;
        let formats = import
            .into_iter()
            .fold(HashMap::new(), |mut formats, format| {
                formats.insert(format.name.clone(), format);

                formats
            });

        Ok(BankFormats { formats })
    }
}
