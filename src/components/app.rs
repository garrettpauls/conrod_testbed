use conrod_core::{widget, Widget, Labelable, Positionable, Sizeable};
use crate::systems::SlowBackgroundProcessor;

widget_ids!(struct Ids {
    background,
    title_label,
    title_text,
    exit_button,
    processed_text,
    processed_text_input,
});

pub struct State {
    ids: Ids,
    text_title: String,
    processed_text: String,
    processed_text_input: String,
}

pub enum AppEvent {
    SetTitle(String),
    Exit,
    Process(String),
}

#[derive(WidgetCommon)]
pub struct App<'a> {
    #[conrod(common_builder)] common: widget::CommonBuilder,
    processor: &'a SlowBackgroundProcessor,
}

impl<'a> App<'a> {
    pub fn new(proc: &'a SlowBackgroundProcessor) -> Self {
        App {
            common: widget::CommonBuilder::default(),
            processor: proc,
        }
    }
}

impl<'a> Widget for App<'a> {
    type State = State;
    type Style = ();
    type Event = Vec<AppEvent>;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
            text_title: String::new(),
            processed_text: String::new(),
            processed_text_input: String::new(),
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

        while let Some(text) = self.processor.next_processed() {
            state.update(|s| {
                s.processed_text += &text;
                s.processed_text += "\n";
            });
        }

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

        for event in widget::TextBox::new(&state.processed_text_input)
            .parent(id)
            .w_h(320.0, 32.0)
            .align_left_of(id)
            .down_from(state.ids.title_text, 4.0)
            .set(state.ids.processed_text_input, ui) {
            use conrod_core::widget::text_box::Event;
            match event {
                Event::Update(text) => state.update(|s| s.processed_text_input = text),
                Event::Enter => {
                    results.push(AppEvent::Process(state.processed_text_input.clone()));
                    state.update(|s| s.processed_text_input.clear());
                }
            }
        }

        let y = ui.xy_of(state.ids.processed_text_input)
            .map(|[_, y]| y)
            .unwrap_or(0.0);
        widget::Text::new(&state.processed_text)
            .parent(id).graphics_for(id)
            .w_of(state.ids.processed_text_input)
            .h(rect.h() - y)
            .set(state.ids.processed_text, ui);

        results
    }
}