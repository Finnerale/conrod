#![allow(missing_docs)]

use color::Color;
use Scalar;

use std::sync::Arc;
use std::path::Path;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use super::*;

pub struct Theme {
    parent: Option<Arc<Theme>>,
    rules: Vec<Rule>,
}

impl Theme {
    pub fn new() -> Self {
        Theme::parse("")
    }

    pub fn parse(s: &str) -> Self {
        Theme {
            parent: None,
            rules: parse(s),
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Theme, String> {
        let file = try!(File::open(path).map_err(|err| format!("failed to open css: {}", err)));
        let mut reader = BufReader::new(file);
        let mut css = String::new();
        let res = reader.read_to_string(&mut css).map_err(|err| format!("failed to read css: {}", err));
        match res {
            Ok(_) => Ok(Theme::parse(&css)),
            Err(err) => Err(err),
        }
    }

    fn all_rules(&self) -> Vec<Rule> {
        if let Some(ref parent) = self.parent {
            self.rules.iter().chain(parent.all_rules().iter()).cloned().collect()
        } else {
            self.rules.clone()
        }
    }

    pub fn get(&self, property: &str, selector: &Selector) -> Option<Value> {
        let mut matches: Vec<(bool, Specificity, Value)> = Vec::new();

        for rule in self.all_rules().iter().rev() {
            let matching_selectors = rule.selectors.iter().filter(|x| x.matches(selector)).collect::<Vec<_>>();

            if matching_selectors.len() > 0 {
                if let Some(decl) = rule.declarations.iter().find(|decl| decl.property == property) {
                    let highest_specifity = matching_selectors.iter().map(|sel| sel.specificity()).max().unwrap();
                    matches.push((decl.important, highest_specifity, decl.value.clone()));
                }
            }
        }

        matches.sort_by_key(|x| (x.0, x.1));
        matches.last().map(|x| x.2.clone())
    }

    pub fn color(&self, property: &str, selector: &Selector) -> Option<Color> {
        self.get(property, selector).map(|v| v.color()).unwrap_or(None)
    }

    pub fn scalar(&self, property: &str, selector: &Selector) -> Option<Scalar> {
        self.get(property, selector).map(|v| v.scalar()).unwrap_or(None)
    }

    pub fn string(&self, property: &str, selector: &Selector) -> Option<String> {
        self.get(property, selector).map(|v| v.string()).unwrap_or(None)
    }
}