use std::collections::HashMap;

use crate::BankTransaction;

#[allow(dead_code)]
pub struct BankTransactions {
    records: HashMap<String, BankTransaction>,
}
