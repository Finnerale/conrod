#![allow(missing_docs)]

use cssparser::{self, Parser, ParseError, ParserInput};

mod theme;
pub use self::theme::Theme;

mod selector;
pub use self::selector::Selector;
use self::selector::{SelectorRelation, Specificity};

mod rule_parser;
use self::rule_parser::RuleParser;

mod declaration_parser;
use self::declaration_parser::DeclarationParser;

mod value;
pub use self::value::{Unit, Value};

#[derive(Clone, Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Clone, Debug)]
pub struct Declaration {
    pub property: String,
    pub value: Value,
    pub important: bool,
}

#[derive(Clone, Debug)]
pub enum CustomParseError {
    InvalidColorName(String),
    InvalidColorHex(String),
}

impl<'t> From<CustomParseError> for ParseError<'t, CustomParseError> {
    fn from(e: CustomParseError) -> Self {
        ParseError::Custom(e)
    }
}

fn parse(s: &str) -> Vec<Rule> {
    let mut input = ParserInput::new(s);
    let mut parser = Parser::new(&mut input);
    let rule_parser = RuleParser::new();

    let rules = {
        let rule_list_parser = cssparser::RuleListParser::new_for_stylesheet(&mut parser, rule_parser);
        rule_list_parser.collect::<Vec<_>>()
    };

    for rule in &rules {
        match *rule {
            Ok(_) => {},
            Err(ref e) => {
                match e.error {
                    ParseError::Basic(ref e) => eprintln!("{:?}", e),
                    ParseError::Custom(ref e) => eprintln!("{:?}", e),
                }
                println!("Error occured in `{}`", parser.slice(e.span.clone()));
            }
        }
    }

    rules.into_iter().filter_map(|rule| rule.ok()).collect()
}

#[test]
fn load_default_theme() {
    use cssparser::CompactCowStr;
    use self::declaration_parser::parse_hex_color;

    let theme = Theme::from_path("./src/css/default_theme.css").unwrap();

    let selector = Selector::new(Some("button"));
    let value = theme.scalar("border-width", &selector);
    assert_eq!(value, 1.0);

    let selector = selector.with_pseudo_class("active");
    let value = theme.color("background", &selector);
    assert_eq!(value, parse_hex_color(CompactCowStr::from("5294E2")).unwrap());
}