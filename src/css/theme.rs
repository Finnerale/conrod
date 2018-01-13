#![allow(missing_docs)]

use color::Color;
use Scalar;

use std::path::Path;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use position;
use label::FontSize;

use super::{parse, Selector, Rule, Value, Specificity};

pub struct Theme {
    rules: Vec<Rule>,
}

impl Theme {
    pub fn new() -> Self {
        Theme::parse("")
    }

    pub fn parse(s: &str) -> Self {
        Theme {
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

    pub fn get(&self, property: &str, selector: &Selector) -> Option<Value> {
        let mut matches: Vec<(bool, Specificity, &Value)> = Vec::new();

        for rule in self.rules.iter().rev() {
            let matching_selectors = rule.selectors.iter().filter(|x| x.matches(selector)).collect::<Vec<_>>();

            if matching_selectors.len() > 0 {
                if let Some(decl) = rule.declarations.iter().find(|decl| decl.property == property) {
                    let highest_specifity = matching_selectors.iter().map(|sel| sel.specificity()).max().unwrap();
                    matches.push((decl.important, highest_specifity, &decl.value));
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

    pub fn font_size(&self, property: &str, selector: &Selector) -> Option<FontSize> {
        self.scalar(property, selector).map(|v| v as FontSize)
    }

    pub fn string(&self, property: &str, selector: &Selector) -> Option<String> {
        self.get(property, selector).map(|v| v.string()).unwrap_or(None)
    }

    pub fn relative_position(&self, property: &str, selector: &Selector) -> Option<position::Relative> {
        self.string(property, selector)
            .map(|string| string.parse::<position::Align>().ok()
            .map(|align| position::Relative::Align(align)))
            .unwrap_or(None)
    }
}