pub use Dimensions;
use widget::Id;

mod constraints;
mod context;
pub mod stack;
pub mod linear;

mod enums;

pub use self::enums::{Layout, LayoutItem};
pub use self::constraints::BoxConstraints;
pub use self::context::LayoutContext;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutResult {
    Size(Dimensions),
    RequestChild(Id, BoxConstraints),
}

pub trait LayoutFunction {

    fn layout(&mut self, constraints: BoxConstraints, children: &[Id], child_size: Option<Dimensions>, context: LayoutContext) -> LayoutResult;

}
