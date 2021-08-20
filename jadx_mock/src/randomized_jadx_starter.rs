use crate::jadx::Jadx;
use orbtk::{
    prelude::{Widget, Window},
    Application,
};
use std::thread;

use crate::theme;

pub fn run_jadx() -> (f64, f64) {
    let width = 800.0;
    let height = 800.0;

    thread::spawn(move || {
        let app = Application::new().theme(theme()).window(move |ctx| {
            Window::new()
                .style("windows_window")
                .title("New Project - jadx-gui")
                .position((0.0, 35.0))
                .size(width, height)
                .resizeable(true)
                .child(Jadx::new().height(1500).width(1500).build(ctx))
                .build(ctx)
        });
        app.run();
    });

    (width, height)
}
