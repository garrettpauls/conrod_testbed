use conrod_core::{widget, Widget, Positionable};

widget_ids!(struct Ids {
    background,
});

pub struct State {
    ids: Ids,
}

#[derive(WidgetCommon)]
pub struct App {
    #[conrod(common_builder)] common: widget::CommonBuilder,
}

impl App {
    pub fn new() -> Self {
        App {
            common: widget::CommonBuilder::default(),
        }
    }
}

impl Widget for App {
    type State = State;
    type Style = ();
    type Event = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
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

        widget::Rectangle::fill_with(rect.dim(), ui.theme.background_color)
            .parent(id).graphics_for(id)
            .xy(rect.xy())
            .set(state.ids.background, ui);
    }
}