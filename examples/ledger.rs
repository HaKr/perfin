use chrono::NaiveDate;
use perfin::{Journal, JournalRepository, Ledger, Result};

use rusty_money::{
    iso::{self, Currency},
    Money,
};

fn main() -> Result<()> {
    let mut ledger = Ledger::load("cb09add43080499a90e7479543e750a9", 2022)?;

    let amount: Money<Currency> = Money::from_minor(100_00, iso::EUR) / 3;

    let journal: &mut dyn Journal = &mut ledger;

    journal.register_single(
        NaiveDate::from_ymd(2022, 01, 26),
        "algemeen",
        &amount,
        "Added by example 1/3",
    )?;
    journal.register_single(
        NaiveDate::from_ymd(2022, 01, 27),
        "algemeen",
        &amount,
        "Added by example 2/3",
    )?;
    journal.register_single(
        NaiveDate::from_ymd(2022, 01, 28),
        "algemeen",
        &amount,
        "Added by example 3/3",
    )?;

    let amount = amount * 3;
    journal.register_single(
        NaiveDate::from_ymd(2022, 01, 28),
        "inkomen",
        &amount,
        "Counter all those spendings",
    )?;

    let journal_repository: &mut dyn JournalRepository = &mut ledger;

    journal_repository.save_journal()?;

    Ok(())
}
