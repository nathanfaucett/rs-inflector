use collections::string::String;

use super::super::regex::Regex;


pub struct Rule {
    pub regex: Regex,
    pub replacer: String,
}

impl Rule {
    pub fn new(rule: String, replacer: String) -> Rule {
        let re = match Regex::new(&rule) {
            Ok(re) => re,
            Err(err) => panic!("{}", err),
        };

        Rule{
            regex: re,
            replacer: replacer,
        }
    }
}
