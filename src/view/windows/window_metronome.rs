use crate::modul;
use egui::*;

#[derive(Default)]
pub struct WindowMetronome {
    is_running: bool,
}

impl WindowMetronome {
    pub fn draw(&mut self, ctx: &egui::Context, modul: &mut modul::Modul, texture: &TextureHandle) {
        let Self { is_running } = self;
        egui::Window::new("metronome").show(ctx, |ui| {
            ctx.request_repaint();
            ui.label(format!("time {}", modul.get_audio_index()));
            let r = ui.checkbox(is_running, "run");
            if r.changed() {
                modul.switch_metronome(*is_running);
            }

            ui.label(format!("{}", modul.get_beat_index()));

            let desired_size = ui.available_width() * vec2(1.0, 0.02);
            let (_id, rect) = ui.allocate_space(desired_size);
            let to_screen =
                emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);
            let mut shapes = vec![];
            ui.image(texture, texture.size_vec2());
            shapes.push(epaint::Shape::circle_filled(
                to_screen * pos2(0.1 + (modul.get_beat_index() % 4) as f32 * 0.05, -4.0),
                10.0,
                if modul.get_beat_index() % 4 == 0 {
                    Color32::RED
                } else {
                    Color32::GREEN
                },
            ));
            ui.painter().extend(shapes);
        });
    }
}
