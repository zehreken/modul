mod visualization;
mod windows;
use std::path::Path;

use self::windows::Windows;

use crate::modul;
use crate::modul::Modul;
use crate::modul_utils::utils::TAPE_COUNT;
use crate::Config;
use egui::Context;
use visualization::material;
use {egui_miniquad as egui_mq, miniquad as mq};

const BABY: (i32, i32, i32, i32) = (178, 177, 178, 169);
const ACID: (i32, i32, i32, i32) = (177, 179, 185, 180);
const BOMB: (i32, i32, i32, i32) = (178, 191, 189, 178);
const ZONE: (i32, i32, i32, i32) = (170, 191, 190, 181);
const WILD: (i32, i32, i32, i32) = (167, 185, 188, 180);
const SOUL: (i32, i32, i32, i32) = (163, 191, 165, 188);
const SELF: (i32, i32, i32, i32) = (163, 181, 188, 182);
const TEXTS: [(i32, i32, i32, i32); 7] = [BABY, ACID, BOMB, ZONE, WILD, SOUL, SELF];

struct Stage {
    small_quad: visualization::Quad,
    big_quad: visualization::Quad,
    windows: windows::Windows,
    modul: modul::Modul,
    egui_mq: egui_mq::EguiMq,
}

impl Stage {
    fn new(mq_ctx: &mut mq::Context, config: Config) -> Self {
        let egui_mq = egui_mq::EguiMq::new(mq_ctx);
        Self {
            small_quad: visualization::Quad::new(mq_ctx, 0.25, 0.5, material::COLOR_BAR),
            big_quad: visualization::Quad::new(mq_ctx, 1.0, 1.0, material::TEXTURE),
            windows: windows::Windows::new(egui_mq.egui_ctx()),
            modul: modul::Modul::new(config),
            egui_mq,
        }
    }
}

impl mq::EventHandler for Stage {
    fn update(&mut self, _ctx: &mut mq::Context) {}

    fn draw(&mut self, ctx: &mut mq::Context) {
        ctx.clear(Some((1., 1., 1., 1.)), None, None);
        ctx.begin_default_pass(mq::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));

        // Draw things behind egui here
        ctx.apply_pipeline(&self.small_quad.pipeline);
        ctx.apply_bindings(&self.small_quad.bindings);

        // Pass data to shader
        for i in 0..TAPE_COUNT {
            ctx.apply_uniforms(&material::Uniforms {
                offset: (
                    -0.75_f32 + (i % 4) as f32 * 0.5_f32,
                    -0.5_f32 + (i / 4) as f32 * 1.0_f32,
                ),
                wavepoint: self.modul.get_sample_averages()[i],
                text: (0, 0, 0, 0),
            });
            ctx.draw(0, 6, 1);
        }

        // Play-through
        if self.modul.is_play_through() {
            let wavepoint = self.modul.get_sample_averages()[8];
            let text = TEXTS[(wavepoint * 1000.0) as usize % 7];
            ctx.apply_pipeline(&self.big_quad.pipeline);
            ctx.apply_bindings(&self.big_quad.bindings);
            ctx.apply_uniforms(&material::Uniforms {
                offset: (0.0, 0.0),
                // 8 is the index of the last element
                wavepoint,
                text,
            });
            ctx.draw(0, 6, 1);
        }
        // ============

        ctx.end_render_pass();

        self.egui_mq.run(ctx, |egui_ctx| {
            draw_ui(&mut self.windows, egui_ctx, &mut self.modul);
        });

        self.egui_mq.draw(ctx);

        // Draw things in front of egui here

        ctx.commit_frame();
    }

    fn mouse_motion_event(&mut self, ctx: &mut mq::Context, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(ctx, x, y);
    }

    fn mouse_wheel_event(&mut self, ctx: &mut mq::Context, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(ctx, dx, dy);
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_down_event(ctx, mb, x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_up_event(ctx, mb, x, y);
    }

    fn char_event(
        &mut self,
        _ctx: &mut mq::Context,
        character: char,
        _keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.char_event(character);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut mq::Context,
        keycode: mq::KeyCode,
        keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.key_down_event(ctx, keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut mq::Context, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }
}

fn draw_ui(windows: &mut Windows, egui_ctx: &Context, modul: &mut Modul) {
    windows.draw(egui_ctx, modul);
}

pub fn start(config: Config) {
    let conf = mq::conf::Conf {
        window_width: 700,
        window_height: 700,
        window_title: "modul".to_string(),
        high_dpi: true,
        ..Default::default()
    };
    mq::start(conf, |mut ctx| {
        mq::UserData::owning(Stage::new(&mut ctx, config), ctx)
    });
}

pub fn load_image(path: &Path) -> image::DynamicImage {
    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let im = image::open(path).expect("image not found");
    println!("atlas image: {}", im.as_bytes().len());
    im
}

fn load_image_for_ui(path: &Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
