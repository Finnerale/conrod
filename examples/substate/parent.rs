
use std::fmt;
use conrod::{widget, Colorable, color, Positionable, Sizeable, Widget};
use child::Child;

#[derive(Default, Debug, Clone, Copy)]
pub struct Item(usize);

impl Item {
    pub fn increase(&mut self) {
        self.0 += 1;
    }

    pub fn get(&self) -> usize {
        self.0
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

widget_ids!(struct Ids {
    sum,
    list,
});

pub struct State {
    ids: Ids,
    items: Vec<Item>,
}

#[derive(WidgetCommon)]
pub struct Parent {
    #[conrod(common_builder)] common: widget::CommonBuilder,
}

impl Parent {
    pub fn new() -> Self {
        Parent {
            common: widget::CommonBuilder::default(),
        }
    }
}

impl Widget for Parent {
    type State = State;
    type Style = ();
    type Event = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
            items: vec![Item::default(); 20],
        }
    }

    fn style(&self) -> Self::Style {}

    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs {
            state,
            ui,
            id,
            rect,
            ..
        } = args;

        let sum: usize = state.items.iter().map(Item::get).sum();

        widget::Text::new(&format!("Sum: {}", sum))
            .h(20.0)
            .mid_top_of(id)
            .color(color::WHITE)
            .set(state.ids.sum, ui);

        let (mut items, scrollbar) = widget::List::flow_down(state.items.len())
            .item_size(40.0)
            .scrollbar_on_top()
            .down(5.0)
            .x(rect.x())
            .w(rect.w())
            .h(rect.h() - 20.0 - 5.0)
            .set(state.ids.list, ui);

        let mut index = 0;
        while let Some(item) = items.next(ui) {
            {
                let child = Child::new(state.substate(|state| state.items.get_mut(index).unwrap()));
                item.set(child, ui);
            }
            index += 1;
        }

        if let Some(s) = scrollbar { s.set(ui) }
    }

}
