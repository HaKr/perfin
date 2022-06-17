use thiserror::Error;

#[derive(Error, Debug)]
pub enum JournalRepositoryError {
    #[error("Could not read configuration from ledger file")]
    ConversionFailed {
        #[from]
        source: serde_yaml::Error,
    },

    #[error("File error")]
    IoError {
        #[from]
        source: std::io::Error,
    },

    #[error("Urecognised account code '{0}'")]
    UnrecognisedAccountCode(String),
}
pub trait JournalRepository {
    fn load_journal(&mut self) -> Result<(), JournalRepositoryError>;
    fn save_journal(&self) -> Result<(), JournalRepositoryError>;
}
