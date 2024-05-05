use super::Drawable;

#[derive(Default)]
pub struct WindowControls {}

impl Drawable for WindowControls {
    fn draw(&mut self, egui_ctx: &egui::Context, _modul: &mut crate::core::Modul) {
        egui::Window::new("controls").show(egui_ctx, |ui| {
            // ctx.request_repaint();
            ui.label("Controls");
            ui.label("select tape ([1-8])");
            ui.label("select secondary tape (shift+[1-8])");
            ui.label("toggle record tape (Space)");
            ui.label("clear tape (C)");
            ui.label("clear all tapes (shift + C)");
            ui.label("mute/unmute tape (M)");
            ui.label("solo/unsolo tape (S)");
            ui.label("merge tapes (N)");
            ui.label("toggle record live (T)");
            ui.label("toggle play-through (Y)");
            ui.label("write to disc (W)");
            ui.label("tape volume (up/down)");
            ui.label("quit (Esc)");
        });
    }
}
