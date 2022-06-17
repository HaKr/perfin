use regex::Regex;

#[derive(Debug)]
pub struct AssignByNameSearch {
    pub account_code: String,
    pub name: String,
    pub search_expression: Regex,
}
