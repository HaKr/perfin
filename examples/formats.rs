use perfin::BankFormats;

fn main() -> Result<(), std::io::Error> {
    let _formats = BankFormats::from_fixture()?;
    // formats.save()
    Ok(())
}
