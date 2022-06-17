use csv::Error as CsvError;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Conversion failed for '{field}'")]
    ConversionFailed {
        field: String,
        #[source]
        source: CsvError,
    },

    #[error("File error")]
    Io {
        #[from]
        source: IoError,
    },

    #[error("CSV error")]
    CsvError {
        #[from]
        source: CsvError,
    },

    #[error("Could not read configuration from ledger file")]
    ConfigurationError {
        #[from]
        source: serde_yaml::Error,
    },

    #[error("incorrect Regex syntax")]
    IncorrectRegex {
        #[from]
        source: regex::Error,
    },

    #[error("Conflicting account codes {by_name} vs {by_ref}")]
    ConflictingAccountCodes { by_name: String, by_ref: String },

    #[error("Error in journal file")]
    JournalRepositoryFileError {
        #[from]
        source: crate::JournalRepositoryError,
    },

    #[error("Urecognised bank account '{0}'")]
    UnrecognisedBankAccount(String),

    #[error("Urecognised cost center code '{0}'")]
    UnrecognisedCostCenterCode(String),

    #[error("Urecognised account code '{0}'")]
    UnrecognisedAccountCode(String),

    #[error("Skipped record #{line_nr}: conversion of '{field}' failed.")]
    RecordConversionFailed { line_nr: usize, field: &'static str },

    #[error("Currency {foreign} must be converted to {ledger}")]
    CurrencyMustBeExchanged { ledger: String, foreign: String },
}

pub type Result<T> = std::result::Result<T, Error>;
