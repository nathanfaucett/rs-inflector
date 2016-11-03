use collections::string::String;

use regex::Regex;


pub struct Rule {
    pub regex: Regex,
    pub replacer: String,
}

impl Rule {
    #[inline]
    pub fn new(rule: String, replacer: String) -> Self {
        let re = match Regex::new(&rule) {
            Ok(re) => re,
            Err(err) => panic!("{}", err),
        };

        Rule {
            regex: re,
            replacer: replacer,
        }
    }
}
