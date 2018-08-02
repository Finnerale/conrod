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

pub trait LayoutFunction {

    fn layout(&mut self, constraints: BoxConstraints, children: &[Id], context: &mut LayoutContext) -> Dimensions;

}
