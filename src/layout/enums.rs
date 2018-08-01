use std::any::Any;
use std::fmt;
use layout::{
    LayoutFunction,
    stack::{Stack, StackItem},
    linear::{Linear, LinearItem},
};

pub enum Layout {
    Stack(Stack),
    Linear(Linear),

    Custom(Box<LayoutFunction + 'static>),
    None,
}

impl fmt::Debug for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Layout::Stack(stack) => stack.fmt(f),
            Layout::Linear(linear) => linear.fmt(f),

            Layout::Custom(_) => write!(f, "Custom(?)"),
            Layout::None => write!(f, "None"),
        }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Layout::None
    }
}

pub enum LayoutItem {
    Stack(StackItem),
    Linear(LinearItem),

    Custom(Box<Any + 'static>),
    None,
}

impl fmt::Debug for LayoutItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LayoutItem::Stack(stack) => stack.fmt(f),
            LayoutItem::Linear(linear) => linear.fmt(f),

            LayoutItem::Custom(_) => write!(f, "Custom(?)"),
            LayoutItem::None => write!(f, "None"),
        }
    }
}

impl Default for LayoutItem {
    fn default() -> Self {
        LayoutItem::None
    }
}
