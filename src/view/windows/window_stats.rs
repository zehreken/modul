use crate::modul;
use std::time::Instant;

pub struct WindowStats {
    instant: Instant,
}

impl Default for WindowStats {
    fn default() -> Self {
        Self {
            instant: Instant::now(),
        }
    }
}

impl WindowStats {
    pub fn draw(&mut self, ctx: &egui::Context, modul: &mut modul::Modul) {
        let Self { instant } = self;
        egui::Window::new("stats").show(ctx, |ui| {
            ctx.request_repaint();
            ui.label(format!(
                "real time: {:0.1} sec",
                instant.elapsed().as_secs_f32()
            ));
            ui.label(format!("modul time: {:0.1}", modul.get_audio_index()));
            ui.label(format!("bar length: {} sec", "not implemented"));
            ui.label(format!("input channels: {}", "not implemented"));
            ui.label(format!("output channels: {}", "not implemented"));
        });
    }
}
