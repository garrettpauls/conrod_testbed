use conrod_core::{widget, Widget, Labelable, Positionable, Sizeable};

widget_ids!(struct Ids {
    background,
    title_label,
    title_text,
    exit_button,
});

pub struct State {
    ids: Ids,
    text_title: String,
}

pub enum AppEvent {
    SetTitle(String),
    Exit,
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
    type Event = Vec<AppEvent>;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
            text_title: String::new(),
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

        let mut results = Vec::new();

        widget::Rectangle::fill_with(rect.dim(), ui.theme.background_color)
            .parent(id).graphics_for(id)
            .xy(rect.xy())
            .set(state.ids.background, ui);

        widget::Text::new("Set title:")
            .parent(id).graphics_for(id)
            .mid_top_with_margin(32.0)
            .w_h(320.0, 32.0)
            .set(state.ids.title_label, ui);

        for event in widget::TextBox::new(&state.text_title)
            .parent(id)
            .wh_of(state.ids.title_label)
            .align_left_of(state.ids.title_label)
            .down_from(state.ids.title_label, 4.0)
            .set(state.ids.title_text, ui) {
            use conrod_core::widget::text_box::Event;
            match event {
                Event::Update(text) => state.update(|s| s.text_title = text),
                Event::Enter => results.push(AppEvent::SetTitle(state.text_title.clone())),
            }
        }

        for _click in widget::Button::new()
            .parent(id)
            .w_h(128.0, 32.0)
            .bottom_right_with_margin_on(id, 4.0)
            .label("Exit")
            .set(state.ids.exit_button, ui) {
            results.push(AppEvent::Exit);
        }

        results
    }
}