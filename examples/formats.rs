use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
};

use chrono::NaiveDate;
use indexmap::IndexMap;
use lazy_regex::regex_replace;
use serde::{Deserialize, Serialize};

use perfin::ing::{DescriptionParser, ParseError};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct IngImport {
    #[serde(rename = "Datum")]
    pub date: String,

    #[serde(rename = "Naam / Omschrijving")]
    pub name: String,

    #[serde(rename = "Rekening")]
    pub iban: String,

    #[serde(rename = "Tegenrekening")]
    pub counter_iban: Option<String>,

    #[serde(rename = "Code")]
    pub code: String,

    #[serde(rename = "Af Bij")]
    pub debit_or_credit: String,

    #[serde(rename = "Mededelingen")]
    pub info: String,

    #[serde(rename = "Bedrag (EUR)")]
    pub amount: String,

    #[serde(rename = "Saldo na mutatie")]
    pub balance_after_transaction: String,

    #[serde(rename = "Tag")]
    pub tag: Option<String>,
}

#[derive(Serialize)]
struct Relations {
    by_iban: HashMap<String, String>,
    relation_names: Vec<String>,
}

fn to_owned(src: IndexMap<&str, &str>) -> IndexMap<String, String> {
    let mut result = IndexMap::new();

    for (key, value) in src.iter() {
        result.insert(key.to_string(), value.to_string());
    }

    result
}

fn main() -> Result<(), ParseError> {
    let args: Vec<String> = env::args().collect();
    let for_year = args[1].as_str();

    let example_files_json = r#"{
        "2018": "01-01-2018_31-12-2018",
        "2019": "01-01-2019_31-12-2019",
        "debug": "2019_debug",
        "2020": "01-01-2020_31-12-2020",
        "2021": "01-01-2021_31-12-2021",
        "2022": "01-01-2022_30-04-2022"
    }"#;
    let example_files: HashMap<&str, &str> = serde_json::from_str(example_files_json).unwrap();
    let example_file = example_files
        .get(for_year)
        .expect("Must specify year 2018-2022");
    let format_config = File::open("data/formats/ing.yaml").unwrap();
    let sample = File::open(format!(
        "data/examples/Alle_rekeningen_{}.csv",
        example_file
    ))
    .unwrap();
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_reader(sample);

    let parser = DescriptionParser::try_from(format_config)?;
    // println!("Parser: {:?}", parser);
    let mut by_iban: HashMap<String, String> = HashMap::new();
    let mut relation_names = HashSet::new();

    for rec in csv_reader.deserialize() {
        let rec: IngImport = rec.unwrap();
        // if rec.code.eq_ignore_ascii_case("BA") {
        let debug = rec.info.contains(";;;;; ");
        let result = parser.parse(rec.code.as_str(), rec.info.as_str(), debug);
        match result {
            Some(result) => {
                let _currency_date = if let Some(currency_date) = result.get("Valutadatum") {
                    NaiveDate::parse_from_str(currency_date, "%d-%m-%Y").unwrap()
                } else {
                    NaiveDate::parse_from_str(rec.date.as_str(), "%Y%m%d").unwrap()
                };

                let counter_iban = if let Some(iban) = result.get("IBAN") {
                    Some(iban.to_string())
                } else {
                    rec.counter_iban
                };

                let relation_name =
                    regex_replace!(r#"^([A-Z\s&]+\*)"#, rec.name.trim(), |_, _prefix| "");

                if let Some(name_from_description) = result.get("Naam") {
                    if !name_from_description.eq_ignore_ascii_case(&relation_name) {
                        return Err(ParseError::NameMismatch {
                            name: relation_name.to_string(),
                            description: rec.info.to_string(),
                            attributes: to_owned(result),
                        });
                    }
                }

                match counter_iban {
                    Some(counter_iban) => {
                        if let Some(existing) = by_iban.get(rec.name.as_str()) {
                            if !rec.name.eq_ignore_ascii_case(existing) {
                                return Err(ParseError::NameClash {
                                    new_name: relation_name.to_string(),
                                    existing: existing.to_string(),
                                    iban: counter_iban,
                                    attributes: to_owned(result),
                                });
                            }
                        } else {
                            by_iban.insert(counter_iban, rec.name.to_string());
                        }
                    }
                    None => {
                        if !["BA", "DV", "GM"].contains(&rec.code.as_str()) {
                            if result.get("Van/Naar").is_none() && result.get("Correctie").is_none()
                            {
                                return Err(ParseError::IbanMissing {
                                    attributes: to_owned(result),
                                });
                            }
                        }
                        relation_names.insert(relation_name.to_string());
                    }
                }
            }
            None => {
                println!(
                    "{}: {}\n\t{:?}",
                    rec.code,
                    rec.info,
                    match result {
                        Some(res) => res,
                        None => IndexMap::new(),
                    }
                );
                break;
            }
        }

        // }
    }

    let mut relation_names = relation_names
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>();
    relation_names.sort();
    let relations = Relations {
        by_iban,
        relation_names,
    };

    serde_yaml::to_writer(std::io::stdout(), &relations).unwrap();
    Ok(())
}
