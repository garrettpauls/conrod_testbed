use conrod_core::{color, theme::Theme};

pub fn default() -> Theme {
    let mut theme = Theme::default();
    theme.name = "Default Theme".to_owned();
    theme.background_color = color::DARK_CHARCOAL;
    theme
}