use Scalar;
use widget::Id;
use layout::{LayoutFunction, LayoutContext, BoxConstraints, Dimensions};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Childless {
    width: Scalar,
    height: Scalar,
}

impl Childless {
    pub fn new() -> Self {
        Childless {
            width: 0.0,
            height: 0.0,
        }
    }

    pub fn width(mut self, width: Scalar) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Scalar) -> Self {
        self.height = height;
        self
    }

    pub fn dimensions(self, dimensions: Dimensions) -> Self {
        self.width(dimensions[0]).height(dimensions[1])
    }
}

impl LayoutFunction for Childless {
    fn layout(&self, constraints: BoxConstraints, children: &[Id], context: &mut LayoutContext) -> Dimensions {
        assert_eq!(children.len(), 0, "A Widget with `Childless` layout is not allowed to have children");

        [
            constraints.check_width(self.width),
            constraints.check_height(self.height),
        ]
    }
}
