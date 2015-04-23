use std::convert::AsRef;
use std::vec::Vec;

mod rule;
use self::rule::Rule;


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

pub fn replace(uncountables: &Vec<String>, replacers: &Vec<Rule>, word: &str) -> String {
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

pub fn create_match_word(word: &str) -> String {
    let mut s = String::new();
    s.push_str("\\b");
    s.push_str(word.clone());
    s.push_str("\\b");
    s
}
