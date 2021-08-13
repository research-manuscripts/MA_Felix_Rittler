mod components;
mod elements;
mod jadx;
mod mock_data_generator;
mod generator_constants;

use crate::jadx::Jadx;
use orbtk::{
    prelude::*,
    widgets::themes::theme_orbtk::{THEME_DEFAULT, THEME_DEFAULT_COLORS_LIGHT, THEME_DEFAULT_FONTS},
};
static CUSTOM_THEME: &str = include_str!("assets/theme/theme.ron");
static CUSTOM_COLORS: &str = include_str!("assets/theme/colors.ron");

fn theme() -> Theme {
    orbtk::widgets::themes::theme_orbtk::register_default_fonts(Theme::from_config(
        ThemeConfig::from(CUSTOM_THEME)
            .extend(ThemeConfig::from(CUSTOM_COLORS))
            .extend(ThemeConfig::from(THEME_DEFAULT))
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_LIGHT))
            .extend(ThemeConfig::from(THEME_DEFAULT_FONTS)),
    ))
}

fn main() {
    let app = Application::new().theme(theme()).window(|ctx| {
        Window::new()
            .style("windows_window")
            .title("New Project - jadx-gui")
            .position((120.0, 120.0))
            .size(800.0, 800.0)
            .resizeable(true)
            .child(Jadx::new().height(1200).width(1500).build(ctx))
            .build(ctx)
    });
    app.run()
}
