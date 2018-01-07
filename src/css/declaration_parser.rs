#![allow(missing_docs)]

use color::{self, Color};
use Scalar;

use cssparser::{self, BasicParseError, CompactCowStr, Parser, ParseError, Token};

use super::{Unit, CustomParseError, Declaration, Value};

pub struct DeclarationParser;

impl<'i> cssparser::DeclarationParser<'i> for DeclarationParser {
    type Declaration = Declaration;
    type Error = CustomParseError;

    fn parse_value<'t>(&mut self, name: CompactCowStr<'i>, input: &mut Parser<'i, 't>) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {

        let value = match input.next()? {
            Token::Ident(s) => Value::Symbol(s.to_string()),
            Token::QuotedString(s) => Value::String(s.to_string()),
            Token::Number { value, .. } => Value::Scalar(value as Scalar, Unit::None),
            Token::Dimension { value, unit, ..} => Value::Scalar(value as Scalar, Unit::from(unit)),
            Token::IDHash(hash) | Token::Hash(hash) => Value::Color(parse_hex_color(hash)?),
            t => {
                let basic_error = BasicParseError::UnexpectedToken(t);
                return Err(basic_error.into());
            }
        };

        Ok(Declaration {
            property: name.into_owned(),
            value: value,
            important: input.try(cssparser::parse_important).is_ok()
        })
    }
}

impl<'i> cssparser::AtRuleParser<'i> for DeclarationParser {
    type Prelude = ();
    type AtRule = Declaration;
    type Error = CustomParseError;
}

pub fn parse_hex_color<'i>(hash: CompactCowStr<'i>) -> Result<Color, ParseError<'i, CustomParseError>> {
    match hash.len() {
        6 | 8 => {
            let mut x = match u32::from_str_radix(&hash, 16) {
                Ok(x) => x,
                Err(_) => return Err(CustomParseError::InvalidColorHex(hash.into_owned()).into()),
            };

            let a;
            let b;
            let g;
            let r;

            if hash.len() == 6 {
                a = 255;
                b = (x & 0xFF) >> 0;
                g = (x & 0xFF00) >> 8;
                r = (x & 0xFF0000) >> 16;
            } else {
                a = (x & 0xFF) >> 0;
                b = (x & 0xFF00) >> 8;
                g = (x & 0xFF0000) >> 16;
                r = (x & 0xFF000000) >> 24;
            }

            let r = (r as f32) / 255.0;
            let g = (g as f32) / 255.0;
            let b = (b as f32) / 255.0;
            let a = (a as f32) / 255.0;

            return Ok(color::rgba(r, g, b, a));
        },
        _ => return Err(CustomParseError::InvalidColorHex(hash.into_owned()).into()),
    }
}