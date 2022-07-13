use super::Drawable;

#[derive(Default)]
pub struct WindowControls {}

impl Drawable for WindowControls {
    fn draw(&mut self, egui_ctx: &egui::Context, _modul: &mut crate::modul::Modul) {
        egui::Window::new("controls").show(egui_ctx, |ui| {
            // ctx.request_repaint();
            ui.label("Controls");
            ui.label("select tape [1-8]");
            ui.label("record tape toggle (R)");
            ui.label("clear tape (C)");
            ui.label("mute/unmute tape (M)");
            ui.label("record live toggle (T)");
            ui.label("play-through toggle (Y)");
            ui.label("write to disc (W)");
            ui.label("tape volume (up/down)");
        });
    }
}
