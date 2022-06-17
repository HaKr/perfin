use chrono::NaiveDate;
use rusty_money::{iso::Currency, Money};

pub trait Journal {
    fn register_single(
        &mut self,
        date: NaiveDate,
        account_code: &str,
        amount: &Money<Currency>,
        description: &str,
    ) -> crate::Result<()>;
}
