use conrod_core::{color, theme::Theme};

pub fn default() -> Theme {
    let mut theme = Theme::default();
    theme.name = "Default Theme".to_owned();
    theme.background_color = color::DARK_CHARCOAL;
    theme.shape_color = color::CHARCOAL;
    theme.label_color = color::LIGHT_GREY;
    theme
}