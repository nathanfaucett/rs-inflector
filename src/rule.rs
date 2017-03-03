use collections::string::String;

use regex::Regex;


pub struct Rule {
    regex: Regex,
    replacer: String,
}

impl Rule {
    #[inline(always)]
    pub fn new(rule: String, replacer: String) -> Self {
        Rule {
            regex: match Regex::new(&rule) {
                Ok(re) => re,
                Err(err) => panic!("{}", err),
            },
            replacer: replacer,
        }
    }
    #[inline(always)]
    pub fn regex(&self) -> &Regex {
        &self.regex
    }
    #[inline(always)]
    pub fn replacer(&self) -> &str {
        &self.replacer
    }
}
