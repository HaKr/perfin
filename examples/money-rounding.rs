use rusty_money::{
    iso::{self, Currency},
    Money,
};

fn main() {
    let amount: Money<Currency> = Money::from_minor(100_00, iso::EUR);
    let x = amount.allocate(vec![3, 3, 3]).unwrap();
    println!("x {:?}", x);

    let amount: Money<Currency> = Money::from_major(100, iso::EUR) / 3;
    assert_eq!(amount.to_string(), "€33,33");

    let triple_amount: Money<Currency> = amount * 3;
    assert_eq!(triple_amount.to_string(), "€99,99");
}
