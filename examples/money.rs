use core::fmt;
use std::{collections::HashSet, io};

use rust_decimal::prelude::*;

#[allow(unused_imports)]
use rusty_money::{iso, Money};
use serde::{de::Unexpected, Serialize};

pub mod int {

    use rust_decimal::Decimal;
    use serde::Serialize;

    use crate::DecimalVisitor;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_any(DecimalVisitor)
    }

    pub fn serialize<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use num_traits::ToPrimitive;
        value.to_i64().unwrap().serialize(serializer)
    }
}

struct DecimalVisitor;

impl<'de> serde::de::Visitor<'de> for DecimalVisitor {
    type Value = Decimal;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a Decimal type representing a fixed-point number"
        )
    }

    fn visit_i64<E>(self, value: i64) -> Result<Decimal, E>
    where
        E: serde::de::Error,
    {
        match Decimal::from_i64(value) {
            Some(s) => Ok(s),
            None => Err(E::invalid_value(Unexpected::Signed(value), &self)),
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Decimal, E>
    where
        E: serde::de::Error,
    {
        match Decimal::from_u64(value) {
            Some(s) => Ok(s),
            None => Err(E::invalid_value(Unexpected::Unsigned(value), &self)),
        }
    }

    fn visit_f64<E>(self, value: f64) -> Result<Decimal, E>
    where
        E: serde::de::Error,
    {
        Decimal::from_str(&value.to_string())
            .map_err(|_| E::invalid_value(Unexpected::Float(value), &self))
    }

    fn visit_str<E>(self, value: &str) -> Result<Decimal, E>
    where
        E: serde::de::Error,
    {
        Decimal::from_str(value)
            .or_else(|_| Decimal::from_scientific(value))
            .map_err(|_| E::invalid_value(Unexpected::Str(value), &self))
    }
}

#[derive(Serialize)]
struct Transaction {
    #[serde(with = "int")]
    amount: Decimal,
    description: String,
    cost_centers: HashSet<String>,
}

fn main() -> serde_yaml::Result<()> {
    let a = Money::from_minor(123456789, iso::EUR);
    println!("Amount = {}", a);

    let mut cost_centers = HashSet::<String>::new();
    cost_centers.insert("Cindy".to_owned());
    cost_centers.insert("Harry".to_owned());

    let t = Transaction {
        amount: a.amount().clone(),
        description: "My first".to_owned(),
        cost_centers,
    };
    serde_yaml::to_writer(io::stdout(), &t)
}
