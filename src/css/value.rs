#![allow(missing_docs)]

use color::Color;
use Scalar;

use cssparser::CompactCowStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    None,
    Px,
    Percent,
}

impl<'i> From<CompactCowStr<'i>> for Unit {
    fn from(s: CompactCowStr<'i>) -> Self {
        match s.as_ref() {
            "px" => Unit::Px,
            "%" => Unit::Percent,
            _ => Unit::None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Color (Color ),
    Scalar(Scalar, Unit),
    Symbol(String),
    String(String),
}

impl Value {
    pub fn scalar(&self) -> Option<Scalar> {
        match *self {
            Value::Scalar(x, _) => Some(x),
            _ => None,
        }
    }

    pub fn color(&self) -> Option<Color> {
        match *self {
            Value::Color(x) => Some(x),
            _ => None,
        }
    }

    pub fn string(&self) -> Option<String> {
        match *self {
            Value::Symbol(ref x) | Value::String(ref x) => Some(x.to_owned()),
            _ => None,
        }
    }
}