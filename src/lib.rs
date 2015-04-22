extern crate regex;
use regex::Regex;

use std::convert::AsRef;
use std::vec::Vec;


struct Rule {
    regex: Regex,
    replacer: String,
}

impl Rule {
    fn new(rule: String, replacer: String) -> Rule {
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


pub struct Inflector {
    locale: String,
    singulars: Vec<Rule>,
    plurals: Vec<Rule>,
    uncountables: Vec<String>,
}

impl Inflector {
    pub fn new(locale: &str) -> Inflector {
        Inflector {
            locale: locale.to_string(),
            singulars: Vec::new(),
            plurals: Vec::new(),
            uncountables: Vec::new(),
        }
    }

    pub fn get_locale(&self) -> String {
        self.locale.to_string()
    }

    pub fn clear(&mut self) -> &mut Inflector {
        self.plurals.clear();
        self.singulars.clear();
        self.uncountables.clear();
        self
    }

    pub fn uncountable(&mut self, uncountables: &[&str]) -> &mut Inflector {
        for uncountable in uncountables {
            self.uncountables.push(uncountable.to_string());
        }
        self
    }

    pub fn plural(&mut self, rule: &str, replacer: &str) -> &mut Inflector {
        self.plurals.push(Rule::new(rule.to_string(), replacer.to_string()));
        self
    }

    pub fn singular(&mut self, rule: &str, replacer: &str) -> &mut Inflector {
        self.singulars.push(Rule::new(rule.to_string(), replacer.to_string()));
        self
    }

    pub fn irregular(&mut self, singular: &str, plural: &str) -> &mut Inflector {
        self.plurals.push(Rule::new(create_match_word(singular), plural.to_string()));
        self.singulars.push(Rule::new(create_match_word(plural), singular.to_string()));
        self
    }

    pub fn pluralize(&mut self, word: &str) -> String {
        replace(&self.uncountables, &self.plurals, word)
    }

    pub fn singularize(&mut self, word: &str) -> String {
        replace(&self.uncountables, &self.singulars, word)
    }
}

fn replace(uncountables: &Vec<String>, replacers: &Vec<Rule>, word: &str) -> String {
    let mut result: String = word.to_string();

    if uncountables.contains(&result) {
        result
    } else {
        for rule in replacers.iter().rev() {
            let regex = &rule.regex;

            if regex.is_match(&result) {
                let replacer: &str = rule.replacer.as_ref();
                result = regex.replace_all(&(result.clone()), replacer);
                break;
            }
        }

        result
    }
}

fn create_match_word(word: &str) -> String {
    let mut s = String::new();
    s.push_str("\\b");
    s.push_str(word.clone());
    s.push_str("\\b");
    s
}

#[test]
fn test_uncountables() {
    let mut inflector = Inflector::new("en");

    inflector.uncountable(&["equipment", "information"]);

    assert!(inflector.singularize("equipment") == "equipment");
    assert!(inflector.pluralize("information") == "information");
}

#[test]
fn test_pluralize() {
    let mut inflector = Inflector::new("en");

    inflector
        .plural("$", "s")
        .plural("(ch|sh|ss|[sxz])$", "$1es")
        .plural("([^aeiouy])y$", "$1ies");

    assert!(inflector.pluralize("car") == "cars");
    assert!(inflector.pluralize("box") == "boxes");
    assert!(inflector.pluralize("sky") == "skies");
}

#[test]
fn test_singularize() {
    let mut inflector = Inflector::new("en");

    inflector
        .singular("s$", "")
        .singular("(ch|sh|ss|[sxz])es$", "$1")
        .singular("([^aeiouy])ies$", "$1y");

    assert!(inflector.singularize("cars") == "car");
    assert!(inflector.singularize("boxes") == "box");
    assert!(inflector.singularize("skies") == "sky");
}
