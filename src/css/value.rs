#![allow(missing_docs)]
#![allow(unused_variables)]

use std::convert::From;

use color::Color;
use Scalar;
use position;
use FontSize;
use text;
use widget;

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

pub struct CssValueable<T: FromValue>(pub T);

pub trait FromValue : Sized {
    fn from(value: Value) -> Self;
}

impl<T> Into<CssValueable<T>> for Value
    where T: FromValue,
{
    fn into(self) -> CssValueable<T> {
        CssValueable(T::from(self))
    }
}

impl FromValue for Option<Color> {
    fn from(value: Value) -> Self {
        value.color()
    }
}

impl FromValue for Option<Scalar> {
    fn from(value: Value) -> Self {
        value.scalar()
    }
}

impl FromValue for Option<FontSize> {
    fn from(value: Value) -> Self {
        value.scalar().map(|it| it as FontSize)
    }
}

impl FromValue for Option<position::Relative> {
    fn from(value: Value) -> Self {
        value.string()
             .map(|string| string.parse::<position::Align>().ok()
             .map(|align| position::Relative::Align(align)))
             .unwrap_or(None)
    }
}

impl FromValue for Option<bool> {
    fn from(value: Value) -> Self {
        value.string()
            .map(|it| {
                match it.as_ref() {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => None
                }
            })
            .unwrap_or(None)
    }
}

// Not realy implemented

impl FromValue for Option<Option<Scalar>> {
    fn from(value: Value) -> Self {
        None
    }
}

impl FromValue for Option<Option<FontSize>> {
    fn from(value: Value) -> Self {
        None
    }
}

impl FromValue for Option<Option<text::font::Id>> {
    fn from(value: Value) -> Self {
        None
    }
}

impl FromValue for Option<text::Justify> {
    fn from(value: Value) -> Self {
        None
    }
}

impl FromValue for Option<widget::primitive::text::Wrap> {
    fn from(value: Value) -> Self {
        None
    }
}
impl FromValue for Option<Option<widget::primitive::text::Wrap>> {
    fn from(value: Value) -> Self {
        None
    }
}

impl FromValue for Option<position::Align> {
    fn from(value: Value) -> Self {
        None
    }
}

impl FromValue for Option<widget::canvas::Style> {
    fn from(value: Value) -> Self {
        None
    }
}

impl FromValue for Option<widget::tabs::Layout> {
    fn from(value: Value) -> Self {
        None
    }
}

impl FromValue for Option<widget::graph::node::SocketLayout> {
    fn from(value: Value) -> Self {
        None
    }
}

impl FromValue for Option<Option<widget::list::ScrollbarPosition>> {
    fn from(value: Value) -> Self {
        None
    }
}

impl FromValue for Option<Option<Color>> {
    fn from(value: Value) -> Self {
        None
    }
}

impl FromValue for Option<Option<widget::drop_down_list::MaxHeight>> {
    fn from(value: Value) -> Self {
        None
    }
}

impl FromValue for Option<widget::canvas::Length> {
    fn from(value: Value) -> Self {
        None
    }
}