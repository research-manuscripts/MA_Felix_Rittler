mod components;
mod elements;
mod generator;
mod jadx;
mod randomized_jadx_starter;
mod screen_capture;

use std::{env, thread, time::{Duration, SystemTime}};

use crate::{jadx::Jadx, randomized_jadx_starter::run_jadx, screen_capture::capture_screen};
use generator::{sample_size, sample_window};
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

fn run_randomized_jadx() {
    let app = Application::new().theme(theme()).window(move |ctx| {
        let additional_window = sample_window();

        let size = sample_size(860..=1500, 740..=1080);

        Window::new()
            .style("windows_window")
            .title("New Project - jadx-gui")
            .position((0.0, 35.0))
            .size(size.width(), size.height())
            .resizeable(true)
            .child(
                Jadx::new()
                    .additional_window(additional_window)
                    .window_height(size.height())
                    .window_width(size.width())
                    .height(1500)
                    .width(1500)
                    .build(ctx),
            )
            .build(ctx)
    });
    app.run();
}

fn main() {
    let time = SystemTime::now();

    let args: Vec<String> = env::args().collect();

    let (width, height) = run_jadx();

    thread::sleep(Duration::from_millis(200));

    capture_screen(format!("target/images/{}.png", args[1]), width as i32, height as i32 + 35);

    let new_sys_time = SystemTime::now();
    let difference = new_sys_time.duration_since(time);
    log::debug!("Time for Screen Capture: {}", difference.unwrap().as_millis());
}
