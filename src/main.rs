use crate::tools::default;
use eframe::egui::Vec2;

mod tools;
mod app;

fn main() {
    let options = eframe::NativeOptions {
        always_on_top: false,
        decorated: true,
        drag_and_drop_support: false,
        icon_data: None,
        initial_window_size: Some(Vec2::new(515.0, 410.0)),
        resizable: false,
        transparent: false,
    };

    eframe::run_native(Box::new(app::State::default()), options);
}
