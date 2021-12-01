use crate::modul;

pub struct WindowMetronome {}

impl Default for WindowMetronome {
    fn default() -> Self {
        Self {}
    }
}

impl WindowMetronome {
    pub fn draw(&mut self, ctx: &egui::CtxRef, modul: &mut modul::Modul) {
        let Self {} = self;
        egui::Window::new("metronome").show(ctx, |ui| {
            ctx.request_repaint();
            ui.label(format!("time {}", 1234567890));
        });
    }
}
