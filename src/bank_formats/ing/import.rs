use std::{collections::HashMap, fs::File, iter::Enumerate, ops::Mul};

use chrono::NaiveDate;
use csv::DeserializeRecordsIter;
use num_traits::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    AccountsRepository, AssignmentReason, BankTransaction, CostCentersRepository,
    RelationsRepository,
};

use super::DescriptionProperties;

pub type Amount = Decimal;

#[derive(Debug, Deserialize)]
enum TransactionCode {
    #[serde(rename = "GT")]
    Girotel,

    #[serde(rename = "GM")]
    ATM,

    #[serde(rename = "BA")]
    PayTerminal,

    #[serde(rename = "DV")]
    Other,

    #[serde(rename = "ID")]
    IDeal,

    #[serde(rename = "IC")]
    Collect,

    #[serde(rename = "VZ")]
    Giro,

    #[serde(rename = "OV")]
    Transfer,
}

#[derive(Debug, Deserialize)]
pub enum DebitOrCredit {
    #[serde(rename = "Af")]
    Credit,

    #[serde(rename = "Bij")]
    Debit,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct IngTransaction {
    pub id: String,
    pub iban: String,
    pub contra_iban: Option<String>,
    pub date: NaiveDate,
    pub balance_before: Amount,
    pub amount: Amount,
    pub balance_after: Amount,
    pub properties: HashMap<String, String>,
}

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
    pub code: TransactionCode,

    #[serde(rename = "Af Bij")]
    pub debit_or_credit: DebitOrCredit,

    #[serde(rename = "Mededelingen")]
    pub info: String,

    #[serde(rename = "Bedrag (EUR)")]
    pub amount: String,

    #[serde(rename = "Saldo na mutatie")]
    pub balance_after_transaction: String,

    #[serde(rename = "Tag")]
    pub tag: Option<String>,
}

pub struct IngImporter<R: std::io::Read> {
    csv_reader: csv::Reader<R>,
}

pub struct IngTransactionIterator<'i, R: std::io::Read> {
    deserializer: Enumerate<DeserializeRecordsIter<'i, R, IngImport>>,
    accounts_repository: &'i dyn AccountsRepository,
    cost_center_repository: &'i dyn CostCentersRepository,
    relations_repository: &'i dyn RelationsRepository,
}

impl IngTransaction {
    pub fn name(&self) -> Option<String> {
        if let Some(name) = self.properties.get(DescriptionProperties::NAME) {
            Some((*name).clone())
        } else {
            None
        }
    }
}

impl IngImporter<File> {
    pub fn try_from_path(path: &str) -> crate::Result<Self> {
        let file_handle = File::open(path)?;

        Ok(Self::from_reader(file_handle))
    }
}

impl<R: std::io::Read> IngImporter<R> {
    pub fn from_reader(rdr: R) -> Self {
        Self {
            csv_reader: csv::ReaderBuilder::new()
                .has_headers(true)
                .delimiter(b';')
                .from_reader(rdr),
        }
    }

    pub fn transactions<'i>(
        &'i mut self,
        accounts_repository: &'i dyn AccountsRepository,
        cost_center_repository: &'i dyn CostCentersRepository,
        relations_repository: &'i dyn RelationsRepository,
    ) -> IngTransactionIterator<'i, R> {
        IngTransactionIterator {
            deserializer: self.csv_reader.deserialize().enumerate(),
            accounts_repository,
            cost_center_repository,
            relations_repository,
        }
    }
}

fn convert_date(
    line_nr: usize,
    field: &'static str,
    date_as_text: &str,
) -> crate::Result<NaiveDate> {
    match NaiveDate::parse_from_str(date_as_text, "%Y%m%d") {
        Ok(date) => Ok(date),
        Err(_) => Err(crate::Error::RecordConversionFailed { line_nr, field }),
    }
}

fn convert_float_with_comma(
    line_nr: usize,
    field: &'static str,
    float_as_text: &str,
) -> crate::Result<Amount> {
    match float_as_text.replace(",", ".").parse::<f64>() {
        Ok(amount) => match Decimal::from_f64(amount) {
            Some(dec) => Ok(
                dec.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
            ),
            None => Err(crate::Error::RecordConversionFailed { line_nr, field }),
        },
        Err(_) => Err(crate::Error::RecordConversionFailed { line_nr, field }),
    }
}

fn convert_record(line_nr: usize, record: IngImport) -> crate::Result<IngTransaction> {
    let date = convert_date(line_nr, "date", &record.date)?;
    let sign = match record.debit_or_credit {
        DebitOrCredit::Credit => Decimal::NEGATIVE_ONE,
        DebitOrCredit::Debit => Decimal::ONE,
    };
    let amount = convert_float_with_comma(line_nr, "amount", &record.amount)?.mul(sign);
    let balance_after =
        convert_float_with_comma(line_nr, "balance", &record.balance_after_transaction)?;

    let balance_before = balance_after - amount;

    let (contra_hash, contra_iban) = if let Some(counter_iban) = &record.counter_iban {
        (counter_iban.as_ref(), Some(counter_iban.clone()))
    } else {
        ("", None)
    };

    let hash_base = format!(
        "|{date}|{iban}|{amount}{dbcr:?}|{counter_iban}|{balance}|{name} {info}|{code:?}|",
        date = record.date,
        amount = record.amount,
        dbcr = record.debit_or_credit,
        name = record.name,
        info = record.info,
        iban = record.iban,
        counter_iban = contra_hash,
        balance = record.balance_after_transaction,
        code = record.code
    );
    let hash = Sha256::digest(hash_base.as_bytes());
    let id = base16ct::lower::encode_string(&hash);

    let mut props = DescriptionProperties::from(record.info.as_str());
    props.define_name(&record.name);

    if let Some(tag) = record.tag {
        props.define_tag(tag)
    };

    Ok(IngTransaction {
        id,
        iban: record.iban,
        contra_iban,
        date,
        balance_before,
        amount,
        balance_after,
        properties: props.properties,
    })
}

impl<'i, R: std::io::Read> Iterator for IngTransactionIterator<'i, R> {
    type Item = crate::Result<BankTransaction>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((record_nr, ing_import_result)) = self.deserializer.next() {
            let line_nr = record_nr + 2;
            if let Err(e) = ing_import_result {
                return Some(Err(crate::Error::from(e)));
            }
            let ing_import_record = ing_import_result.unwrap();

            match convert_record(line_nr, ing_import_record) {
                Ok(ing_transaction) => {
                    let mut attributes = ing_transaction.properties;
                    if let Some(cost_center) = self
                        .cost_center_repository
                        .find_cost_center_by_iban(&ing_transaction.iban)
                    {
                        let mut assignment_reason = None;
                        let mut account_code = None;

                        let (relation_iban, relation_name_from_iban) =
                            if let Some(counter_iban) = ing_transaction.contra_iban {
                                match self
                                    .relations_repository
                                    .find_relation_by_reference(counter_iban.as_str())
                                {
                                    Some(relation) => {
                                        (Some(counter_iban.clone()), Some(relation.name.clone()))
                                    }
                                    None => (None, None),
                                }
                            } else {
                                (None, None)
                            };

                        if let Some(contract_code) = attributes.get(DescriptionProperties::CONTRACT)
                        {
                            if let Some(assign_by_contract) = self
                                .accounts_repository
                                .search_account_by_contract(contract_code)
                            {
                                attributes.remove(DescriptionProperties::CONTRACT);
                                assignment_reason = Some(AssignmentReason::Contract);
                                account_code = Some(assign_by_contract.account_code.clone());
                                let existing = attributes.get(DescriptionProperties::DESCRIPTION);
                                let proposed = assign_by_contract.description.clone();

                                if let Some(description) = if proposed.is_some() {
                                    let proposed = proposed.unwrap();
                                    if existing.is_some() {
                                        Some(format!("{} {}", proposed, existing.unwrap()))
                                    } else {
                                        Some(format!("{}", proposed))
                                    }
                                } else {
                                    if existing.is_some() {
                                        Some(format!("{}", existing.unwrap()))
                                    } else {
                                        None
                                    }
                                } {
                                    attributes.insert(
                                        DescriptionProperties::DESCRIPTION.to_string(),
                                        description,
                                    );
                                }
                            }
                        }

                        if let Some(tag) = attributes.get(DescriptionProperties::TAG) {
                            if account_code.is_none() {
                                if let Some(account) = self
                                    .accounts_repository
                                    .find_account_by_reference(tag.as_str())
                                {
                                    assignment_reason = Some(AssignmentReason::Reference);
                                    account_code = Some(account.code.clone())
                                }
                            }
                            attributes.remove(DescriptionProperties::TAG);
                        };

                        let mut relation_name =
                            if let Some(relation_name_from_iban) = relation_name_from_iban {
                                Some(relation_name_from_iban)
                            } else {
                                attributes.remove(DescriptionProperties::NAME)
                            };

                        if let Some(search_relation_name) = relation_name.clone() {
                            let key = format!("{} & {}", cost_center, search_relation_name);
                            if let Some(description) =
                                attributes.get(DescriptionProperties::DESCRIPTION)
                            {
                                if let Some(assign_by_definition) = self
                                    .accounts_repository
                                    .search_account_by_description(key.as_str(), description)
                                {
                                    account_code = Some(assign_by_definition.account_code.clone());
                                    assignment_reason = Some(AssignmentReason::Description);
                                }
                            }
                            if account_code.is_none() {
                                if let Some((assign_by_name_search, full_relation_name)) = self
                                    .accounts_repository
                                    .search_account_by_name(search_relation_name.as_str())
                                {
                                    relation_name = Some(full_relation_name);
                                    if account_code.is_none() {
                                        assignment_reason = Some(AssignmentReason::RelationName);

                                        account_code =
                                            Some(assign_by_name_search.account_code.clone());
                                    }
                                }
                            }
                        }

                        Some(Ok(BankTransaction {
                            id: ing_transaction.id,
                            date: ing_transaction.date,
                            cost_center,
                            relation_iban,
                            relation_name,
                            attributes,
                            amount: ing_transaction.amount,
                            account_code,
                            assignment_reason,
                        }))
                    } else {
                        None
                    }
                }
                Err(err) => Some(Err(err)),
            }
        } else {
            None
        }
    }
}
