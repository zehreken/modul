use crate::core::Modul;
use crate::core::*;
use egui::*;
use std::path::Path;

use super::Drawable;

pub struct WindowMetronome {
    is_running: bool,
    texture: egui::TextureHandle,
}

impl WindowMetronome {
    pub fn new(ctx: &egui::Context) -> Self {
        let image = load_image_for_ui(Path::new("assets/ui/square.jpg")).unwrap();
        let mut texture_handle: Option<TextureHandle> = Option::None;
        let texture: &egui::TextureHandle = texture_handle.get_or_insert_with(|| {
            ctx.load_texture("square", image.clone(), TextureOptions::default())
        });
        Self {
            is_running: false,
            texture: texture.clone(),
        }
    }
}
impl Drawable for WindowMetronome {
    fn draw(&mut self, ctx: &egui::Context, modul: &mut Modul) {
        let Self {
            is_running,
            texture,
        } = self;

        egui::Window::new("metronome").show(ctx, |ui| {
            ctx.request_repaint();
            ui.label(format!("time: {}", modul.get_audio_index()));
            ui.label("sign: 4/4");
            ui.checkbox(is_running, "beep").changed();
            {
                modul.switch_metronome(*is_running);
            }

            let desired_size = ui.available_width() * vec2(1.0, 0.02);
            let (_id, rect) = ui.allocate_space(desired_size);
            let to_screen =
                emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);
            let mut shapes = vec![];
            let s = texture.size_vec2();
            let id = texture.id();
            // ui.horizontal(|ui| {
            //     ui.image(id, s);
            //     ui.image(id, s);
            //     ui.image(id, s);
            //     ui.image(id, s);
            // });
            shapes.push(epaint::Shape::circle_filled(
                to_screen * pos2(0.03 + (modul.get_beat_index() % 4) as f32 * 0.082, 5.0),
                9.0,
                if modul.get_beat_index() % 4 == 0 {
                    Color32::RED
                } else {
                    Color32::BLUE
                },
            ));
            ui.painter().extend(shapes);
        });
    }
}
