use conrod::{widget, Widget, Labelable, Positionable, Sizeable};

use parent::Item;

widget_ids!(struct Ids {
    button
});

pub struct State {
    ids: Ids
}

#[derive(WidgetCommon)]
pub struct Child<'a> {
    #[conrod(common_builder)] common: widget::CommonBuilder,
    substate: widget::State<'a, Item>
}

impl<'a> Child<'a> {
    pub fn new(substate: widget::State<'a, Item>) -> Self {
        Child {
            common: widget::CommonBuilder::default(),
            substate,
        }
    }
}

impl<'a> Widget for Child<'a> {
    type State = State;
    type Style = ();
    type Event = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
        }
    }

    fn style(&self) -> Self::Style {}

    fn update(mut self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs {
            state,
            ui,
            id,
            ..
        } = args;

        for _click in widget::Button::new()
            .parent(id)
            .top_left_of(id)
            .wh_of(id)
            .label(&format!("Count: {}", *self.substate))
            .set(state.ids.button, ui)
        {
            self.substate.update(|substate| substate.increase());
        }
    }

}
