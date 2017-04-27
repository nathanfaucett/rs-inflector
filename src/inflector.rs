use collections::string::{String, ToString};
use collections::vec::Vec;

use rule::Rule;


pub struct Inflector {
    locale: String,
    singulars: Vec<Rule>,
    plurals: Vec<Rule>,
    uncountables: Vec<String>,
}

impl Inflector {
    
    #[inline(always)]
    pub fn new(locale: &str) -> Self {
        Inflector {
            locale: locale.to_string(),
            singulars: Vec::new(),
            plurals: Vec::new(),
            uncountables: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn locale(&self) -> &String {
        &self.locale
    }

    #[inline]
    pub fn clear(&mut self) -> &mut Self {
        self.plurals.clear();
        self.singulars.clear();
        self.uncountables.clear();
        self
    }

    #[inline]
    pub fn uncountable(&mut self, uncountables: &[&str]) -> &mut Self {
        for uncountable in uncountables {
            self.uncountables.push(uncountable.to_string());
        }
        self
    }

    #[inline]
    pub fn plural(&mut self, rule: &str, replacer: &str) -> &mut Self {
        self.plurals.push(Rule::new(rule.to_string(), replacer.to_string()));
        self
    }

    #[inline]
    pub fn singular(&mut self, rule: &str, replacer: &str) -> &mut Self {
        self.singulars.push(Rule::new(rule.to_string(), replacer.to_string()));
        self
    }

    #[inline]
    pub fn irregular(&mut self, singular: &str, plural: &str) -> &mut Self {
        self.plurals.push(
            Rule::new(Self::create_match_word(singular), plural.to_string())
        );
        self.singulars.push(
            Rule::new(Self::create_match_word(plural), singular.to_string())
        );
        self
    }

    #[inline(always)]
    pub fn pluralize(&self, word: &str) -> String {
        Self::replace(&self.uncountables, &self.plurals, word)
    }

    #[inline(always)]
    pub fn singularize(&self, word: &str) -> String {
       Self::replace(&self.uncountables, &self.singulars, word)
    }

    #[inline]
    fn replace(uncountables: &Vec<String>, replacers: &Vec<Rule>, word: &str) -> String {
        let result = word.to_string();

        if uncountables.contains(&result) {
            result
        } else {
            for rule in replacers.iter().rev() {
                let regex = rule.regex();

                if regex.is_match(&result) {
                    return regex.replace_all(&result, rule.replacer()).to_string();
                }
            }

            result
        }
    }

    #[inline]
    fn create_match_word(word: &str) -> String {
        let mut s = String::new();
        s.push_str("\\b");
        s.push_str(&word.to_lowercase());
        s.push_str("\\b");
        s
    }
}
