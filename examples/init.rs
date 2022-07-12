use perfin::Ledger;

fn main() {
    let ledger = Ledger::default();

    serde_yaml::to_writer(std::io::stdout(), &ledger).unwrap();
}
