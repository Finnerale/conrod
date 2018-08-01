use Scalar;
use widget::Id;
use layout::{LayoutFunction, LayoutContext, LayoutResult, LayoutItem, BoxConstraints, Dimensions};

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

    index: usize,
    step: Step,
    growing_children: Scalar,
    occuppied_length: Scalar,
    widest_child: Scalar,
}

#[derive(Debug, Clone, Copy)]
pub struct LinearItem {
    grow: bool,
}

impl Linear {
    pub fn new() -> Self {
        Linear {
            direction: Direction::Vertical,

            index: 0,
            step: Step::SizeFixed,
            growing_children: 0.0,
            occuppied_length: 0.0,
            widest_child: 0.0,
        }
    }

    fn update_widest(&mut self, size: Dimensions) {
        let index = match self.direction {
            Direction::Horizontal => 1,
            Direction::Vertical => 0,
        };

        if self.widest_child < size[index] {
            self.widest_child = size[index];
        }
    }

    fn max_width(&self, constraints: BoxConstraints) -> Scalar {
        match self.direction {
            Direction::Horizontal => constraints.max_width - self.occuppied_length,
            Direction::Vertical => constraints.max_width,
        }
    }

    fn max_height(&self, constraints: BoxConstraints) -> Scalar {
        match self.direction {
            Direction::Horizontal => constraints.max_height,
            Direction::Vertical => constraints.max_height - self.occuppied_length,
        }
    }

    fn max_length(&self, constraints: BoxConstraints) -> Scalar {
        match self.direction {
            Direction::Horizontal => constraints.max_width,
            Direction::Vertical => constraints.max_height,
        }
    }

    fn growing_constraints(&self, constraints: BoxConstraints) -> BoxConstraints {
        match self.direction {
            Direction::Horizontal => BoxConstraints::default()
                .max_height(constraints.max_height)
                .fit_width((constraints.max_width - self.occuppied_length) / self.growing_children),
            Direction::Vertical => BoxConstraints::default()
                .fit_height((constraints.max_height - self.occuppied_length) / self.growing_children)
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
    fn layout(&mut self, constraints: BoxConstraints, children: &[Id], child_size: Option<Dimensions>, mut context: LayoutContext) -> LayoutResult {
        loop {
            if let Some(size) = child_size {
                self.update_widest(size);
                self.occuppied_length += match self.direction {
                    Direction::Horizontal => size[0],
                    Direction::Vertical => size[1],
                };
            }
            if self.step == Step::SizeFixed || self.step == Step::SizeGrowing {
                if self.occuppied_length >= self.max_length(constraints) {
                    self.step = Step::Position;
                }
            }
            match self.step {
                Step::SizeFixed => {
                    if self.index < children.len() {
                        let child = children[self.index];
                        self.index += 1;

                        let item = match context.layout_item(child) {
                            LayoutItem::Linear(item) => *item,
                            _ => LinearItem::new(),
                        };

                        if item.grow {
                            self.growing_children += 1.0;
                            continue
                        }

                        let child_constraints = BoxConstraints::default()
                            .max_width(self.max_width(constraints))
                            .max_height(self.max_height(constraints));

                        LayoutResult::RequestChild(child, child_constraints);

                    } else {
                        self.index = 0;
                        self.step = Step::SizeGrowing;
                    }
                }
                Step::SizeGrowing => {
                    if self.index < children.len() {
                        let child = children[self.index];
                        self.index += 1;

                        let item = match context.layout_item(child) {
                            LayoutItem::Linear(item) => *item,
                            _ => LinearItem::new(),
                        };

                        if !item.grow { continue }

                        LayoutResult::RequestChild(child, self.growing_constraints(constraints));

                    } else {
                        self.index = 0;
                        self.step = Step::Position;
                    }
                }
                Step::Position => {
                    self.occuppied_length = 0.0;
                    for child in children.iter().cloned() {
                        let size = context.get_size(child);

                        if let Some(size) = size {
                            match self.direction {
                                Direction::Horizontal => {
                                    context.position(child, [self.occuppied_length, 0.0]);
                                    self.occuppied_length += size[0];
                                }
                                Direction::Vertical => {
                                    context.position(child, [0.0, self.occuppied_length]);
                                    self.occuppied_length += size[1];
                                }
                            }
                        }
                    }
                    self.step = Step::Finished;
                }
                Step::Finished => {
                    return LayoutResult::Size( match self.direction {
                        Direction::Horizontal => [
                            constraints.check_width(self.occuppied_length),
                            constraints.check_height(self.widest_child),
                        ],
                        Direction::Vertical => [
                            constraints.check_width(self.widest_child),
                            constraints.check_height(self.occuppied_length),
                        ],
                    } )
                }
            }
        }
    }
}
