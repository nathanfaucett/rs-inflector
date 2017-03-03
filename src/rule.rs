use collections::string::String;

use regex::{Regex, RegexBuilder};


pub struct Rule {
    regex: Regex,
    replacer: String,
}

impl Rule {
    #[inline(always)]
    pub fn new(rule: String, replacer: String) -> Self {
        Rule {
            regex: {
                RegexBuilder::new(&rule)
                    .case_insensitive(true)
                    .unicode(true)
                    .build()
                    .unwrap()
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
