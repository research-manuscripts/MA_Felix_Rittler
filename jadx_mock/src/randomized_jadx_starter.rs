use crate::{
    generator::{sample_size, sample_window},
    jadx::{Jadx},
};
use orbtk::{
    prelude::{Widget, Window},
    Application,
};
use std::thread;

use crate::theme;

pub fn run_jadx() -> (f64, f64) {
    let size = sample_size(860..=1500, 740..=1080);

    thread::spawn(move || {
        let app = Application::new().theme(theme()).window(move |ctx| {
            let additional_window = sample_window();

            Window::new()
                .style("windows_window")
                .title("New Project - jadx-gui")
                .position((0.0, 35.0))
                .size(size.width(), size.height())
                .resizeable(true)
                .child(
                    Jadx::new()
                        .additional_window(additional_window)
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

    (size.width(), size.height())
}
