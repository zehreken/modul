use super::Drawable;
use crate::modul;

#[derive(Default)]
pub struct WindowLog {}

impl Drawable for WindowLog {
    fn draw(&mut self, egui_ctx: &egui::Context, modul: &mut modul::Modul) {
        egui::Window::new("log").show(egui_ctx, |ui| {
            ui.label("logs");
            for message in &modul.message_history {
                ui.label(message.to_string());
            }
        });
    }
}
