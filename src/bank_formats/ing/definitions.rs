#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;

use indexmap::IndexMap;
use regex::Regex;
use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FormatDefinitions {
    aliases: HashMap<String, String>,
    definitions_per_mutation_kind: HashMap<String, Vec<SequenceDefinition>>,
}

type SequenceDefinition = IndexMap<String, String>;
type Sequence = Vec<RegexElement>;

#[derive(Default, Debug)]
pub struct DescriptionParser {
    alternative_contents_per_mutation_kind: HashMap<String, Vec<Sequence>>,
}

#[derive(Debug)]
struct RegexElement {
    is_optional: bool,
    re: Regex,
    name: String,
}

impl RegexElement {
    fn replace_alias_and_compile(
        aliases: &HashMap<String, String>,
        value: (&String, &String),
    ) -> Result<Self, ParseError> {
        let alias: Regex = Regex::new("~([^~]+)~").unwrap();
        let (name, re_src) = value;
        let is_optional = name.contains("?");
        let without_name = name.contains("^");

        let real_name = name.replace("^", "").replace("?", "");

        let re_src = if without_name {
            format!("({})$", re_src)
        } else {
            format!("{}: ({})$", real_name, re_src)
        };

        let re_src = if alias.is_match(re_src.as_str()) {
            let mut src_without_alias = re_src;
            while alias.is_match(src_without_alias.as_ref()) {
                let src_with_alias = src_without_alias.clone();
                let src_with_alias = src_with_alias.as_str();
                for capture in alias.captures_iter(src_with_alias) {
                    let locations = capture.get(0).unwrap();
                    let prefix = &src_with_alias[0..locations.start()];
                    let suffix =
                        &src_with_alias[usize::min(locations.end(), src_with_alias.len())..];
                    if let Some(alias_name) = capture.get(1) {
                        let alias_name = alias_name.as_str();
                        if let Some(alias_value) = aliases.get(alias_name) {
                            src_without_alias =
                                format!("{}{}{}", prefix, alias_value.to_owned(), suffix);
                        } else {
                            return Err(ParseError::AliasNotDefined {
                                alias: alias_name.to_string(),
                                definition: format!("{}: {}", value.0, value.1),
                            });
                        }
                    }
                }
            }
            src_without_alias
        } else {
            re_src
        };

        Ok(Self {
            is_optional,
            re: Regex::new(re_src.as_str())?,
            name: real_name,
        })
    }
}

impl DescriptionParser {
    pub fn try_from<R: std::io::Read>(src: R) -> Result<Self, ParseError> {
        let definitions = FormatDefinitions::try_from(src)?;
        let mut alternative_contents_per_mutation_kind = HashMap::new();

        for (kind, definition) in definitions.definitions_per_mutation_kind.iter() {
            let sequences = definition;
            let mut alternatives = vec![];
            for regex_src in sequences.iter() {
                let mut regex_elements = vec![];
                for src in regex_src.iter().rev() {
                    regex_elements.push(RegexElement::replace_alias_and_compile(
                        &definitions.aliases,
                        src,
                    )?);
                }
                alternatives.push(regex_elements);
            }

            alternative_contents_per_mutation_kind.insert(kind.to_owned(), alternatives);
        }
        Ok(Self {
            alternative_contents_per_mutation_kind,
        })
    }

    pub fn parse<'d>(
        &'d self,
        kind: &str,
        description: &'d str,
        is_debug: bool,
    ) -> Option<IndexMap<&'d str, &'d str>> {
        if let Some(definitions) = self.alternative_contents_per_mutation_kind.get(kind) {
            let mut result: IndexMap<&str, &str>;

            for definition in definitions.iter() {
                result = IndexMap::with_capacity(definition.len());
                let mut description_src = description;
                for element in definition.iter() {
                    if is_debug {
                        println!("Element: {} ({:?})", element.name, element.re);
                        println!(
                            "\t{},{}",
                            element.re.is_match(description_src),
                            description_src
                        );
                    }
                    if let Some(found) = element.re.captures(description_src) {
                        let complete = found.get(0).unwrap();
                        let found = found.get(1).unwrap();
                        if is_debug {
                            println!("Found: {}", &description_src[found.start()..found.end()]);
                        }
                        result.insert(
                            element.name.as_str(),
                            &description_src[found.start()..found.end()],
                        );
                        if is_debug {
                            println!("Remainder: {}", description_src[0..complete.start()].trim());
                        }
                        description_src = description_src[0..complete.start()].trim();
                    } else if !element.is_optional {
                        result.clear();
                        break;
                    }
                }
                if result.len() > 0 {
                    return Some(result);
                }
            }
        }

        None
    }
}

impl FormatDefinitions {
    pub fn try_from<R: std::io::Read>(src: R) -> Result<Self, ParseError> {
        let result: Self = serde_yaml::from_reader(src)?;
        Ok(result)
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Incorrect stream format")]
    IncorrectFormat {
        #[from]
        source: serde_yaml::Error,
    },

    #[error("")]
    RegexDefinitionError {
        #[from]
        source: regex::Error,
    },

    #[error("Field '{name}' does not match name attribute in description '{description}'\n{attributes:?}")]
    NameMismatch {
        name: String,
        description: String,
        attributes: IndexMap<String, String>,
    },

    #[error("'{existing}' was registered for {iban}, but now '{new_name}' wants to take it's place\n{attributes:?}")]
    NameClash {
        new_name: String,
        existing: String,
        iban: String,
        attributes: IndexMap<String, String>,
    },

    #[error("IBAN is missing\n{attributes:?}")]
    IbanMissing {
        attributes: IndexMap<String, String>,
    },

    #[error("Alias '{alias}' is not defined")]
    AliasNotDefined { alias: String, definition: String },
}
