#![allow(missing_docs)]

use cssparser::{self, BasicParseError, DeclarationListParser, Parser, ParseError, Token};
use std::mem;

use super::{SelectorRelation, CustomParseError, Rule, DeclarationParser, Selector};

pub struct RuleParser;

impl RuleParser {
    pub fn new() -> Self {
        RuleParser {}
    }
}

impl<'i> cssparser::QualifiedRuleParser<'i> for RuleParser {
    type Prelude = Vec<Selector>;
    type QualifiedRule = Rule;
    type Error = CustomParseError;

    fn parse_prelude<'t>(&mut self, input: &mut Parser<'i, 't>)
        -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let res = parse_selectors(input)?;
        Ok(res)
    }

    fn parse_block<'t>(&mut self, selectors: Self::Prelude, input: &mut Parser<'i, 't>)
        -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let decl_parser = DeclarationParser {};

        let decls = DeclarationListParser::new(input, decl_parser).collect::<Vec<_>>();

        for decl in &decls {
            match *decl {
                Ok(_) => {},
                Err(ref e) => {
                    match e.error {
                        ParseError::Basic(ref e) => eprintln!("{:?}", e),
                        ParseError::Custom(ref e) => eprintln!("{:?}", e),
                    }
                    println!("Error occured in `{}`", input.slice(e.span.clone()));
                }
            }
        }

        let decls = decls.into_iter().filter_map(|decl| decl.ok()).collect();

        Ok(Rule {
            selectors: selectors,
            declarations: decls,
        })
    }
}

impl<'i> cssparser::AtRuleParser<'i> for RuleParser {
    type Prelude = ();
    type AtRule = Rule;
    type Error = CustomParseError;
}

fn parse_selectors<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Vec<Selector>, ParseError<'i, CustomParseError>> {
    let mut selectors = Vec::new();

    let mut selector = Selector::default();

    let mut first_token_in_selector = true;
    while let Ok(t) = input.next() {
        match t {
            // Element
            Token::Ident(ref element_name) => {
                if first_token_in_selector {
                    selector.element = Some(element_name.to_string())
                } else {
                    let mut old_selector = Selector::new(Some(element_name.to_string()));
                    mem::swap(&mut old_selector, &mut selector);
                    selector.relation = Some(Box::new(SelectorRelation::Ancestor(old_selector)));
                }
            }

            Token::Delim('>') => {
                let mut old_selector = Selector::new(Some(input.expect_ident()?.to_string()));
                mem::swap(&mut old_selector, &mut selector);
                selector.relation = Some(Box::new(SelectorRelation::Parent(old_selector)));
            }

            // Any element
            Token::Delim('*') => {}

            // Class
            Token::Delim('.') => {selector.classes.insert(input.expect_ident()?.into_owned());}

            // Pseudo-class
            Token::Colon => {selector.pseudo_classes.insert(input.expect_ident()?.into_owned());}

            // This selector is done, on to the next one
            Token::Comma => {
                selectors.push(selector);
                selector = Selector::default();
                first_token_in_selector = true;
                continue; // need to continue to avoid `first_token_in_selector` being set to false
            }

            t => {
                let basic_error = BasicParseError::UnexpectedToken(t);
                return Err(basic_error.into());
            }
        }

        first_token_in_selector = false;
    }

    selectors.push(selector);

    if selectors.iter().any(|sel| sel.relation.is_some()) {
        eprintln!("WARNING: Complex selector relations not implemented");
    }

    Ok(selectors)
}