use std::fs::File;

use perfin::{ing::{ParseError, DescriptionParser}};

fn main() -> Result<(), ParseError> {
    let format_config = File::open("data/formats/ing.yaml").unwrap();

    let parser = DescriptionParser::try_from(format_config)?;

    Ok(())
}
