mod components;
mod elements;
mod generator;
mod jadx;
mod randomized_jadx_starter;
mod screen_capture;

use std::{env, fs, thread, time::{Duration, SystemTime}};
use crate::{randomized_jadx_starter::start_jadx, screen_capture::capture_screen};
use orbtk::{
    prelude::*,
    widgets::themes::theme_orbtk::{THEME_DEFAULT, THEME_DEFAULT_COLORS_LIGHT, THEME_DEFAULT_FONTS},
};
use randomized_jadx_starter::run_randomized_jadx;

static CUSTOM_THEME: &str = include_str!("assets/theme/theme.ron");
static CUSTOM_COLORS: &str = include_str!("assets/theme/colors.ron");

///
/// Loads the OrbTK theme files (default and custom themes and colors)
fn theme() -> Theme {
    orbtk::widgets::themes::theme_orbtk::register_default_fonts(Theme::from_config(
        ThemeConfig::from(CUSTOM_THEME)
            .extend(ThemeConfig::from(CUSTOM_COLORS))
            .extend(ThemeConfig::from(THEME_DEFAULT))
            .extend(ThemeConfig::from(THEME_DEFAULT_COLORS_LIGHT))
            .extend(ThemeConfig::from(THEME_DEFAULT_FONTS)),
    ))
}

///
/// Amount of time in milliseconds between running the JADX mock and taking the screenshot.
/// Depends on the power of your local machine and the window opening animations
/// of you host OS. If the screenshots are taken before JADX mock is loaded,
/// increase this time span.
const TIME_BEFORE_SCREENSHOT: u64 = 200;

///
/// Starts the generator to generate a number of images into 'target/images'
/// First command line argument has to be the number of generated images.
fn main() {
    let time = SystemTime::now();

    // create target folder
    match fs::create_dir_all("target/images") {
        Ok(_) => {}
        Err(_) => print!("Error: Cannot create target directory!"),
    }

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let size = start_jadx();
        thread::sleep(Duration::from_millis(TIME_BEFORE_SCREENSHOT));
        capture_screen(format!("target/images/{}.png", args[1]), size.width() as i32, size.height() as i32 + 35);

        let new_sys_time = SystemTime::now();
        let difference = new_sys_time.duration_since(time);
        log::debug!("Time for Screen Capture: {}", difference.unwrap().as_millis());
    } else {
        run_randomized_jadx();
    }
}
