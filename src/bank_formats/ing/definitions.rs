#![allow(dead_code)]
use std::collections::HashMap;

use indexmap::IndexMap;
use regex::Regex;
use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct FormatDefinitions {
    aliases: HashMap<String, String>,
    definitions_per_mutation_kind: HashMap<String, FormatDefinition>,
} 

type SequenceDefinition = IndexMap<String,String>;
type Sequence = IndexMap<String,Regex>;

#[derive(Deserialize, Debug)]
enum FormatDefinition {
    #[serde(rename="sequence")]
    Sequence(SequenceDefinition)
}

#[derive(Default)]
pub struct DescriptionParser {
    definitions_per_mutation_kind: HashMap<String,Vec<Sequence>>
}

impl DescriptionParser  {
    pub fn try_from<R: std::io::Read>( src: R ) -> Result<Self, ParseError> {
        let definitions = FormatDefinitions::try_from(src)?;
        println!("Defs: {:?}", definitions);
        Ok(Self::default())
    }

    pub fn parse<'d>(&'d self, kind: &str, description: &'d str ) -> Option<IndexMap<&'d str,&'d str>> {

        if let Some(definitions ) = self.definitions_per_mutation_kind.get(kind) {
            let mut result = IndexMap::new();

            if result.len() > 0 {
                return Some(result)
            } 
        } 
        
        None
    }
}

impl FormatDefinitions  {
    pub fn try_from<R: std::io::Read>( src: R ) -> Result<Self, ParseError> {
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

}

