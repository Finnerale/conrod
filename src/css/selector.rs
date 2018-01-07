#![allow(missing_docs)]

use fnv::FnvHashSet;
use std::ops::Add;

/// Describes the specificity of a selector.
///
/// The indexes are as follows:
/// 0 - number of IDs (most important)
/// 1 - number of classes and pseudo-classes
/// 2 - number of elements (least important)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Specificity([u8; 3]);

impl Add<Self> for Specificity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Specificity([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

#[derive(Clone, Debug)]
pub enum SelectorRelation {
    Ancestor(Selector),
    Parent(Selector),
}

#[derive(Clone, Debug, Default)]
pub struct Selector {
    pub element: Option<String>,
    pub classes: FnvHashSet<String>,
    pub pseudo_classes: FnvHashSet<String>,
    pub relation: Option<Box<SelectorRelation>>,
}

impl Selector {
    pub fn new<S: Into<String>>(element: Option<S>) -> Self {
        Selector {
            element: element.map(|s| s.into()),
            classes: FnvHashSet::default(),
            pseudo_classes: FnvHashSet::default(),
            relation: None,
        }
    }

    pub(crate) fn specificity(&self) -> Specificity {
        let s = Specificity([
            0,
            (self.classes.len() + self.pseudo_classes.len()) as u8,
            if self.element.is_some() { 1 } else { 0 }
        ]);

        if let Some(ref relation) = self.relation {
            match **relation {
                SelectorRelation::Ancestor(ref x) | SelectorRelation::Parent(ref x) => return x.specificity() + s,
            }
        }

        s
    }

    pub fn matches(&self, other: &Selector) -> bool {
        if self.element.is_some() && self.element != other.element {
            return false;
        }

        if !other.classes.is_superset(&self.classes) {
            return false;
        }

        if !other.pseudo_classes.is_superset(&self.pseudo_classes) {
            return false;
        }

        true
    }

    pub fn with_class<S: Into<String>>(&mut self, class: S) {
        self.classes.insert(class.into());
    }

    pub fn without_class<S: Into<String>>(&mut self, class: S) {
        self.classes.remove(&class.into());
    }

    pub fn with_pseudo_class<S: Into<String>>(&mut self, pseudo_class: S) {
        self.pseudo_classes.insert(pseudo_class.into());
    }

    pub fn without_pseudo_class<S: Into<String>>(&mut self, pseudo_class: S) {
        self.pseudo_classes.remove(&pseudo_class.into());
    }
}

impl Selector {
    pub fn is_empty(&self) -> bool {
        self.element.is_none() && self.classes.is_empty() && self.pseudo_classes.is_empty()
    }
}

impl<T: Into<String>> From<T> for Selector {
    fn from(t: T) -> Self {
        Selector::new(Some(t.into()))
    }
}