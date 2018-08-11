pub use Dimensions;
use widget::Id;

mod enums;
pub use self::enums::{Layout, LayoutItem};

mod context;
pub use self::context::LayoutContext;

mod constraints;
pub use self::constraints::BoxConstraints;

mod stack;
pub use self::stack::{Stack, StackItem};

mod linear;
pub use self::linear::{Linear, LinearItem};

mod childless;
pub use self::childless::Childless;

pub trait LayoutFunction {

    fn layout(&self, constraints: BoxConstraints, children: &[Id], context: &mut LayoutContext) -> Dimensions;

}

impl LayoutFunction for () {

    fn layout(&self, constraints: BoxConstraints, children: &[Id], context: &mut LayoutContext) -> Dimensions {
        assert!(children.len() <= 1, "A Widget with `()` layout is not allowed to have more than one child");

        if children.len() == 0 {
            [
                constraints.check_width(0.0),
                constraints.check_height(0.0),
            ]
        } else {
            context.size(children[0], constraints)
        }


    }

}
