use conrod_core::theme::Theme;

pub fn default() -> Theme {
    let mut theme = Theme::default();
    theme.name = "Default Theme".to_owned();
    theme
}