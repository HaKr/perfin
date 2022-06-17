use std::{
    collections::{HashMap, HashSet},
    fs::File,
};

use regex::Regex;
use rust_decimal::RoundingStrategy;
use rusty_money::iso::{self, Currency};
use serde::{Deserialize, Serialize};

use crate::{
    Account, AccountHibernate, AccountsRepository, AssignByContractDefinition, AssignByDescription,
    AssignByDescriptionDefinition, AssignByNameSearch, BankAccount, BankFormat,
    CostCentersRepository, Error, Journal, JournalEntry, JournalRepository, Relation,
    RelationsRepository, Result,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ledger {
    #[serde(default)]
    id: String,
    name: String,
    #[serde(default)]
    year: u32,
    currency_iso: String,
    #[serde(skip, default = "default_currency")]
    currency: &'static Currency,

    bank_formats: HashMap<String, BankFormat>,
    cost_centers: HashSet<String>,
    bank_accounts: HashMap<String, BankAccount>,
    relations: HashMap<String, Relation>,
    pub(crate) accounts: HashMap<String, Account>,

    #[serde(rename = "assign_by_name")]
    assign_by_name_definition: HashMap<String, Vec<String>>,
    #[serde(rename = "assign_by_description")]
    assign_by_description_definition: HashMap<String, Vec<AssignByDescriptionDefinition>>,

    assign_by_contract: HashMap<String, AssignByContractDefinition>,

    #[serde(skip)]
    accounts_by_name: Vec<AssignByNameSearch>,
    #[serde(skip)]
    pub assign_by_description: HashMap<String, Vec<AssignByDescription>>,

    #[serde(skip)]
    pub journal: Vec<JournalEntry>,
}

impl Ledger {
    pub fn load(organisation_id: &str, year: u32) -> Result<Self> {
        let ledger_file = File::open(format!(
            "./data/organisations/{}/{}/ledger.yaml",
            organisation_id, year
        ))?;
        let mut result: Self = serde_yaml::from_reader(ledger_file)?;
        result.id = organisation_id.to_owned();
        result.year = year;
        if let Some(currency) = iso::find(result.currency_iso.as_str()) {
            result.currency = currency;
        }

        for (iban, bank_account) in result.bank_accounts.iter_mut() {
            bank_account.iban = iban.to_owned();
            if !result.cost_centers.contains(&bank_account.cost_center_code) {
                return Err(Error::UnrecognisedCostCenterCode(
                    bank_account.cost_center_code.clone(),
                ));
            }
        }

        for (account_code, account) in result.accounts.iter_mut() {
            account.code = account_code.to_owned();
        }

        for (relation_reference, relation) in result.relations.iter_mut() {
            relation.reference = relation_reference.clone();
        }

        result.accounts_by_name = vec![];

        for (account_code, search_terms) in result.assign_by_name_definition.iter() {
            let account = result.find_account_by_reference(account_code.as_str());
            let account_code = account_code.to_string();
            if account.is_none() {
                return Err(Error::UnrecognisedAccountCode(account_code));
            }
            for search_term in search_terms {
                let search_expression = Regex::new(
                    format!(
                        "(?i){}",
                        search_term.replace(r#"{naam}"#, r#"\w+(\s+\w+)?"#)
                    )
                    .as_str(),
                )?;
                result.accounts_by_name.push(AssignByNameSearch {
                    account_code: account_code.clone(),
                    name: search_term.clone(),
                    search_expression,
                })
            }
        }

        for (key, search_list) in result.assign_by_description_definition.iter() {
            // let account = result.find_account_by_reference(account_code.as_str());
            let mut list = vec![];
            for assign_definition in search_list.iter() {
                match result.find_account_by_reference(assign_definition.account_code.as_str()) {
                    Some(_) => {
                        let assign_by_description =
                            AssignByDescription::try_from(assign_definition)?;
                        list.push(assign_by_description);
                    }
                    None => {
                        return Err(Error::UnrecognisedAccountCode(
                            assign_definition.account_code.clone(),
                        ))
                    }
                }
            }

            result.assign_by_description.insert(key.to_string(), list);
        }

        result.load_journal()?;

        Ok(result)
    }

    pub fn accounts_for_hibernate(&self) -> Vec<AccountHibernate> {
        self.accounts
            .values()
            .map(|account_model| AccountHibernate::from(account_model))
            .collect()
    }

    pub fn file_name(&self, base_name: &str) -> String {
        format!(
            "./data/organisations/{}/{}/{}",
            self.id, self.year, base_name
        )
    }
}

impl Journal for Ledger {
    fn register_single(
        &mut self,
        date: chrono::NaiveDate,
        account_code: &str,
        amount: &rusty_money::Money<Currency>,
        description: &str,
    ) -> crate::Result<()> {
        if amount.currency() != self.currency {
            return Err(Error::CurrencyMustBeExchanged {
                ledger: self.currency.to_string(),
                foreign: amount.currency().to_string(),
            });
        }

        if !self.accounts.contains_key(account_code) {
            return Err(Error::UnrecognisedAccountCode(account_code.to_string()));
        }

        let decimal = amount.amount();
        let amount = decimal.round_dp_with_strategy(
            self.currency.exponent,
            RoundingStrategy::MidpointAwayFromZero,
        );

        self.journal.push(JournalEntry {
            date,
            account_code: account_code.to_string(),
            amount,
            description: description.to_string(),
        });

        Ok(())
    }
}

fn default_currency() -> &'static Currency {
    iso::USD
}

impl JournalRepository for Ledger {
    fn load_journal(&mut self) -> std::result::Result<(), crate::JournalRepositoryError> {
        let journal_file = File::open(self.file_name("journal.yaml"))?;
        {
            let mut journal: Vec<JournalEntry> = serde_yaml::from_reader(journal_file)?;
            for journal_entry in journal.iter_mut() {
                if !self.accounts.contains_key(&journal_entry.account_code) {
                    return Err(crate::JournalRepositoryError::UnrecognisedAccountCode(
                        journal_entry.account_code.to_owned(),
                    ));
                }
                journal_entry.amount = journal_entry.amount.round_dp_with_strategy(
                    self.currency.exponent,
                    RoundingStrategy::MidpointAwayFromZero,
                );
            }
            self.journal = journal;
        }
        Ok(())
    }

    fn save_journal(&self) -> std::result::Result<(), crate::JournalRepositoryError> {
        let transactions_file = File::create(self.file_name("journal.yaml"))?;

        serde_yaml::to_writer(transactions_file, &self.journal)?;

        Ok(())
    }
}

impl CostCentersRepository for Ledger {
    fn find_cost_center_by_iban(&self, iban: &str) -> Option<String> {
        self.bank_accounts.iter().find_map(|(_, bank_account)| {
            if bank_account.iban.eq_ignore_ascii_case(iban) {
                Some(bank_account.cost_center_code.clone())
            } else {
                None
            }
        })
    }
}

impl AccountsRepository for Ledger {
    fn find_account_by_reference(&self, reference: &str) -> Option<&Account> {
        self.accounts.get(reference)
    }

    fn search_account_by_name(&self, name: &str) -> Option<(&AssignByNameSearch, String)> {
        self.accounts_by_name
            .iter()
            .find_map(|account_by_name_search| {
                if let Some(capture) = account_by_name_search.search_expression.captures(name) {
                    Some((account_by_name_search, capture[0].to_string()))
                } else {
                    None
                }
            })
    }

    fn search_account_by_contract(
        &self,
        contract_code: &str,
    ) -> Option<&AssignByContractDefinition> {
        self.assign_by_contract.get(contract_code)
    }

    fn search_account_by_description(
        &self,
        key: &str,
        description: &str,
    ) -> Option<&AssignByDescription> {
        match self.assign_by_description.get(key) {
            Some(search_list) => search_list.iter().find(|assign_by_description| {
                assign_by_description
                    .search_expression
                    .is_match(description)
            }),
            None => None,
        }
    }
}

impl RelationsRepository for Ledger {
    fn find_relation_by_reference(&self, reference: &str) -> Option<&Relation> {
        self.relations.get(reference)
    }
}
