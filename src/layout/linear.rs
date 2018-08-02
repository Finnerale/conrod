use Scalar;
use widget::Id;
use layout::{LayoutFunction, LayoutContext, LayoutItem, BoxConstraints, Dimensions};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    SizeFixed,
    SizeGrowing,
    Position,
    Finished,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy)]
pub struct Linear {
    direction: Direction,
}

#[derive(Debug, Clone, Copy)]
pub struct LinearItem {
    grow: bool,
}

impl Linear {
    pub fn new() -> Self {
        Linear {
            direction: Direction::Vertical,
        }
    }

    fn max_width(&self, constraints: BoxConstraints, occuppied_length: Scalar) -> Scalar {
        match self.direction {
            Direction::Horizontal => constraints.max_width - occuppied_length,
            Direction::Vertical => constraints.max_width,
        }
    }

    fn max_height(&self, constraints: BoxConstraints, occuppied_length: Scalar) -> Scalar {
        match self.direction {
            Direction::Horizontal => constraints.max_height,
            Direction::Vertical => constraints.max_height - occuppied_length,
        }
    }

    fn max_length(&self, constraints: BoxConstraints) -> Scalar {
        match self.direction {
            Direction::Horizontal => constraints.max_width,
            Direction::Vertical => constraints.max_height,
        }
    }

    fn growing_constraints(&self, constraints: BoxConstraints, occuppied_length: Scalar, growing_children: Scalar) -> BoxConstraints {
        match self.direction {
            Direction::Horizontal => BoxConstraints::default()
                .max_height(constraints.max_height)
                .fit_width((constraints.max_width - occuppied_length) / growing_children),
            Direction::Vertical => BoxConstraints::default()
                .fit_height((constraints.max_height - occuppied_length) / growing_children)
                .max_width(constraints.max_width),
        }
    }
}

impl LinearItem {
    pub fn new() -> Self {
        LinearItem {
            grow: false,
        }
    }

    pub fn grow(mut self) -> Self {
        self.grow = true;
        self
    }
}

impl LayoutFunction for Linear {
    fn layout(&mut self, constraints: BoxConstraints, children: &[Id], context: &mut LayoutContext) -> Dimensions {
        let mut growing_children = 0.0;
        let mut occuppied_length = 0.0;
        let mut widest_child = 0.0;

        for child in children.iter().cloned() {
            let item = match context.layout_item(child) {
                LayoutItem::Linear(item) => *item,
                _ => LinearItem::new(),
            };

            if item.grow {
                growing_children += 1.0;
                continue
            }

            let child_constraints = BoxConstraints::default()
                .max_width(self.max_width(constraints, occuppied_length))
                .max_height(self.max_height(constraints, occuppied_length));

            let size = context.size(child, child_constraints);

            let index = match self.direction {
                Direction::Horizontal => 1,
                Direction::Vertical => 0,
            };

            if widest_child < size[index] {
                widest_child = size[index];
            }

            occuppied_length += match self.direction {
                Direction::Horizontal => size[0],
                Direction::Vertical => size[1],
            };
        }

        for child in children.iter().cloned() {
            let item = match context.layout_item(child) {
                LayoutItem::Linear(item) => *item,
                _ => LinearItem::new(),
            };

            if !item.grow { continue }

            let size = context.size(child, self.growing_constraints(constraints, occuppied_length, growing_children));

            let index = match self.direction {
                Direction::Horizontal => 1,
                Direction::Vertical => 0,
            };

            if widest_child < size[index] {
                widest_child = size[index];
            }
        }

        occuppied_length = 0.0;
        for child in children.iter().cloned() {
            let size = context.get_size(child);

            if let Some(size) = size {
                match self.direction {
                    Direction::Horizontal => {
                        context.position(child, [occuppied_length, 0.0]);
                        occuppied_length += size[0];
                    }
                    Direction::Vertical => {
                        context.position(child, [0.0, occuppied_length]);
                        occuppied_length += size[1];
                    }
                }
            }
        }

        match self.direction {
            Direction::Horizontal => [
                constraints.check_width(occuppied_length),
                constraints.check_height(widest_child),
            ],
            Direction::Vertical => [
                constraints.check_width(widest_child),
                constraints.check_height(occuppied_length),
            ],
        }
    }
}
