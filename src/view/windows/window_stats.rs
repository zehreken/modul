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
            ui.label(format!("time: {:0.1} sec", instant.elapsed().as_secs_f32()));
            ui.label(format!("audio index: {:0.1}", modul.get_audio_index()));
            ui.label(format!("bpm: {}", modul.stats.bpm));
            ui.label(format!("bar count: {}", modul.stats.bar_count));
            ui.label(format!("bar length: {} sec", modul.stats.bar_length));
            ui.label(format!(
                "input channel count: {}",
                modul.stats.input_channel_count
            ));
            ui.label(format!(
                "input buffer size: {}",
                modul.stats.input_buffer_size
            ));
            ui.label(format!(
                "output channel count: {}",
                modul.stats.output_channel_count
            ));
            ui.label(format!(
                "output buffer size: {}",
                modul.stats.output_buffer_size
            ));
        });
    }
}
