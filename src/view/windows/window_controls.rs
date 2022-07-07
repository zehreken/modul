#[derive(Default)]
pub struct WindowControls {}

impl WindowControls {
    pub fn draw(&mut self, ctx: &egui::Context) {
        egui::Window::new("controls").show(ctx, |ui| {
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
