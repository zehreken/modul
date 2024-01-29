use egui::{Color32, RichText};

use crate::core::Modul;

use super::Drawable;

#[derive(Default)]
pub struct WindowStats {}

impl Drawable for WindowStats {
    fn draw(&mut self, egui_ctx: &egui::Context, modul: &mut Modul) {
        let instant = modul.instant;
        egui::Window::new("stats").show(egui_ctx, |ui| {
            egui_ctx.request_repaint();
            ui.label(RichText::new(format!("FPS: {0:0.2}", modul.stats.fps)).color(Color32::RED));
            ui.label(format!("time: {:0.1} sec", instant.elapsed().as_secs_f32()));
            ui.label(format!("audio index: {:0.1}", modul.get_audio_index()));
            ui.label(format!("bpm: {}", modul.stats.bpm));
            ui.label(format!("bar count: {}", modul.stats.bar_count));
            ui.label(format!("bar length: {} sec", modul.stats.bar_length));
            ui.colored_label(
                Color32::from_rgb(255, 0, 55),
                format!("input device: {}", modul.stats.input_device_name),
            );
            ui.label(format!(
                "input channel count: {}",
                modul.stats.input_channel_count
            ));
            ui.label(format!(
                "input buffer size: {}",
                modul.stats.input_buffer_size
            ));
            ui.colored_label(
                Color32::from_rgb(255, 00, 55),
                format!("output device: {}", modul.stats.output_device_name),
            );
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
