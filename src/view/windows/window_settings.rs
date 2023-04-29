use super::{Drawable, Modul};

#[derive(Default)]
pub struct WindowSettings {}

impl Drawable for WindowSettings {
    fn draw(&mut self, egui_ctx: &egui::Context, modul: &mut Modul) {
        egui::Window::new("Settings").show(egui_ctx, |ui| {
            ui.label("Settings");
        });
    }
}
