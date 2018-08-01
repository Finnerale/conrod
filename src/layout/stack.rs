use position::Align;
use widget::Id;
use layout::{LayoutFunction, LayoutContext, LayoutResult, BoxConstraints, Dimensions};

#[derive(Debug, Clone, Copy)]
pub struct Stack {
    index: usize,
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
            index: 0,
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
    fn layout(&mut self, constraints: BoxConstraints, children: &[Id], _child_size: Option<Dimensions>, _context: LayoutContext) -> LayoutResult {
        if self.index < children.len() {
            self.index += 1;
            let child_constraints = constraints.grow_to_max();
            LayoutResult::RequestChild(children[self.index-1], child_constraints)
        } else {
            LayoutResult::Size([constraints.max_width, constraints.max_height])
        }
    }
}
