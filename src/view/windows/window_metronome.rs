use crate::modul;
use egui::*;

pub struct WindowMetronome {
    is_running: bool,
}

impl Default for WindowMetronome {
    fn default() -> Self {
        Self { is_running: false }
    }
}

impl WindowMetronome {
    pub fn draw(&mut self, ctx: &egui::CtxRef, modul: &mut modul::Modul) {
        let Self { is_running } = self;
        egui::Window::new("metronome").show(ctx, |ui| {
            ctx.request_repaint();
            ui.label(format!("time {}", 1234567890));
            let r = ui.checkbox(is_running, "run");
            if r.changed() {
                modul.switch_metronome(*is_running);
            }

            ui.label(if modul.show_beat() { "dup" } else { "" });

            let desired_size = ui.available_width() * vec2(1.0, 0.02);
            let (_id, rect) = ui.allocate_space(desired_size);
            let to_screen =
                emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);
            let mut shapes = vec![];
            shapes.push(epaint::Shape::circle_filled(
                to_screen * pos2(0.0, 0.0),
                10.0,
                Color32::from_rgb(0, 255, 0),
            ));
            ui.painter().extend(shapes);
        });
    }
}
