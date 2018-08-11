use position::Align;
use widget::Id;
use layout::{LayoutFunction, LayoutContext, BoxConstraints, Dimensions};

#[derive(Debug, Clone, Copy)]
pub struct Stack {
    align_x: Align,
    align_y: Align,
}

#[derive(Debug, Clone, Copy)]
pub struct StackItem {
    grow: bool,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            align_x: Align::Middle,
            align_y: Align::Middle,
        }
    }
}

impl StackItem {
    pub fn new() -> Self {
        StackItem {
            grow: true,
        }
    }
}

impl LayoutFunction for Stack {
    fn layout(&self, constraints: BoxConstraints, children: &[Id], context: &mut LayoutContext) -> Dimensions {
        for child in children {
            let child_constraints = constraints.grow_to_max();
            context.size(*child, child_constraints);
        }

        [constraints.max_width, constraints.max_height]
    }
}
