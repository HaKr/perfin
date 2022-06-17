use std::{collections::HashMap, fmt::Display};

use regex::Regex;

pub struct DescriptionProperties {
    pub properties: HashMap<String, String>,
}

impl DescriptionProperties {
    pub const DESCRIPTION: &'static str = "Omschrijving";
    pub const NAME: &'static str = "Naam";
    pub const TAG: &'static str = "Tag";
    pub const CONTRACT: &'static str = "Machtiging ID";

    pub fn define_name(&mut self, val: &str) {
        let adjusted = if val.starts_with("CCV") {
            &val[3..]
        } else {
            val
        };
        self.properties
            .insert(Self::NAME.to_owned(), adjusted.to_owned());
    }

    pub fn define_tag(&mut self, val: String) {
        let val = if val.starts_with('#') {
            val[1..].to_owned()
        } else {
            val
        };
        self.properties.insert(Self::TAG.to_owned(), val);
    }

    pub fn define_description(&mut self, val: String) {
        self.properties.insert(Self::DESCRIPTION.to_owned(), val);
    }

    pub fn description(&self) -> Option<&String> {
        self.properties.get(Self::DESCRIPTION)
    }
}

impl Display for DescriptionProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let count = self
            .properties
            .iter()
            .filter(|(n, _)| !Self::DESCRIPTION.eq_ignore_ascii_case(n))
            .count();
        let last_index = if count > 0 { count - 1 } else { 0 };

        if let Some(description) = self.description() {
            f.write_str(description)?;

            if count > 0 {
                f.write_str(", ")?
            }
        }

        for (index, (name, val)) in self
            .properties
            .iter()
            .filter(|(n, _)| !Self::DESCRIPTION.eq_ignore_ascii_case(n))
            .enumerate()
        {
            write!(f, "{}: {}", name, val)?;
            if index < last_index {
                f.write_str(", ")?
            }
        }
        Ok(())
    }
}

impl From<&str> for DescriptionProperties {
    fn from(info: &str) -> Self {
        let ignore: [&str; 4] = ["IBAN", "Valutadatum", "Incassant ID", "Check"];
        // let debug = "Naam: GBLT Omschrijving: GBLT incasso maandelijkse termijn Termijn 8 van 10 Vervaldatum 28 feb 2022 Betaalkenmerk 87419536 Heffingjaar 2021 IBAN: NL82DEUT0319804615 Kenmerk: K2BGBL000000087378701 Machtiging ID: 11740450 Incassant ID: NL07ZZZ082053570000 Doorlopende incasso".eq_ignore_ascii_case(info);
        let re = Regex::new(r"[a-zA-Z][^\s]+( ID)?:\s*").unwrap();
        let name_re = Regex::new(r"[^:]+").unwrap();
        let n = info.len();
        let bounds: Vec<(usize, usize, usize)> = re
            .find_iter(info)
            .map(|m| (m.start(), m.end(), n))
            .collect();

        let first_start = if let Some((first_start, _, _)) = &bounds.first() {
            if *first_start > 1 {
                Some(*first_start)
            } else {
                None
            }
        } else {
            None
        };

        let mut bound: Option<usize> = None;
        let mut result = Self {
            properties: bounds
                .into_iter()
                .rev()
                .filter_map(|(start, mid, end)| {
                    let name = name_re.find(&info[start..mid]).unwrap().as_str();
                    assert!(name.len() > 0, "No name");

                    let mut str_end = match bound {
                        Some(e) => e,
                        None => end,
                    };
                    bound = Some(start);

                    if ignore
                        .iter()
                        .find(|ignore| ignore.eq_ignore_ascii_case(name))
                        .is_some()
                    {
                        return None;
                    }

                    let val = &info[mid..str_end];
                    for c in val.chars().rev() {
                        if str_end > mid {
                            if c.is_ascii_whitespace() {
                                str_end -= 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    let val = &info[mid..str_end];

                    if val.len() > 0 {
                        Some((name, val))
                    } else {
                        None
                    }
                })
                .fold(
                    HashMap::<String, String>::new(),
                    |mut props, (name, val)| {
                        props.insert(name.to_owned(), val.to_owned());
                        props
                    },
                ),
        };

        if let Some(first_start) = first_start {
            let intro = &info[..first_start].trim();
            let description = if let Some(descr) = result.description() {
                format!("{} {}", intro, descr)
            } else {
                intro.to_owned().to_owned()
            };
            result.define_description(description);
        }

        result
    }
}
