mod components;
mod elements;
mod generator;
mod jadx;
mod screen_capture;
mod randomized_jadx_starter;

use std::{env, thread, time::{Duration, SystemTime}};

use crate::{randomized_jadx_starter::run_jadx};
use orbtk::{
    prelude::*,
    widgets::themes::theme_orbtk::{THEME_DEFAULT, THEME_DEFAULT_COLORS_LIGHT, THEME_DEFAULT_FONTS},
};
use screen_capture::capture_screen;

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

// fn main() {
//     let app = Application::new().theme(theme()).window(move |ctx| {
//         Window::new()
//             .style("windows_window")
//             .title("New Project - jadx-gui")
//             .position((0.0, 35.0))
//             .size(800, 800)
//             .resizeable(true)
//             .child(Jadx::new().height(1500).width(1500).build(ctx))
//             .build(ctx)
//     });
//     app.run();
// }

fn main() {
    let time = SystemTime::now();

    let args: Vec<String> = env::args().collect();

    let (width, height) = run_jadx();

    thread::sleep(Duration::from_millis(200));

    capture_screen(format!("target/images/{}.png", args[1]), width as i32, height as i32 + 35);

    let new_sys_time = SystemTime::now();
    let difference = new_sys_time.duration_since(time);
    println!("Time for Screen Capture: {}", difference.unwrap().as_millis());
}
