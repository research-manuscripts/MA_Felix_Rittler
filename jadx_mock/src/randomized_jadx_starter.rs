use crate::{
    components::TopMenuType,
    generator::{sample_size, sample_top_menu, sample_window},
    jadx::{Jadx, WindowType},
};
use orbtk::{Application, prelude::{Size, Widget, Window}};
use std::thread;

use crate::theme;

///
/// Starts a JADX instance with random content in a new thread.
/// Returns the window size.
pub fn start_jadx() -> Size{
    let size = Size::new(900.0, 900.0);

    thread::spawn(move || {
        let app = Application::new().theme(theme()).window(move |ctx| {
            let additional_window = sample_window();

            // when there is no additional window opened, sample a top menu
            let top_menu = if additional_window == WindowType::None {
                sample_top_menu()
            } else {
                TopMenuType::None
            };

            Window::new()
                .style("windows_window")
                .title("New Project - jadx-gui")
                .position((0.0, 35.0))
                .size(size.width(), size.height())
                .resizeable(true)
                .child(
                    Jadx::new()
                        .additional_window(additional_window)
                        .opened_menu(top_menu)
                        .height(1500)
                        .width(1500)
                        .window_height(size.height())
                        .window_width(size.width())
                        .build(ctx),
                )
                .build(ctx)
        });
        app.run();
    });

    size
}


///
/// Runs a JADX instance with random content. Only for debug purposes.
pub fn run_randomized_jadx() {
    let app = Application::new().theme(theme()).window(move |ctx| {
        let additional_window = sample_window();

        // let size = sample_size(860..=1500, 740..=1080);
        let size = Size::new(900.0, 900.0);
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
