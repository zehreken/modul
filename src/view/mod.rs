mod visualization;
mod windows;
use glam::{vec3, EulerRot, Mat4, Quat, Vec3};
use std::path::Path;

use self::{visualization::object::Object, windows::Windows};

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
const GTFO: (i32, i32, i32, i32) = (183, 164, 182, 191);
const TEXTS: [(i32, i32, i32, i32); 7] = [BABY, ACID, BOMB, ZONE, WILD, SOUL, GTFO];

struct Stage {
    quads: Vec<Object>,
    big_quad: Object,
    cube: Object,
    _test_obj: Object,
    windows: windows::Windows,
    modul: modul::Modul,
    egui_mq: egui_mq::EguiMq,
    rotation: f32,
}

impl Stage {
    fn new(mq_ctx: &mut mq::Context, config: Config) -> Self {
        let egui_mq = egui_mq::EguiMq::new(mq_ctx);
        let mut quads = Vec::with_capacity(TAPE_COUNT);
        for i in 0..TAPE_COUNT {
            quads.push(
                Object::new(mq_ctx, material::SDF_EYE)
                    .position(Vec3::new(
                        -2.25 + (i % 4) as f32 * 1.5_f32,
                        -0.75_f32 + (i / 4) as f32 * 1.5_f32,
                        0.0,
                    ))
                    .scale(Vec3::new(0.75, 0.75, 0.75))
                    .build(),
            );
        }
        Self {
            quads,
            big_quad: Object::new(mq_ctx, material::_TEXTURE).build(),
            // cube: visualization::cube::Cube::new(mq_ctx, 1.0, 1.0, material::SDF_CIRCLE),
            cube: Object::new(mq_ctx, material::SDF_CIRCLE)
                .shape(Box::new(visualization::cube::Cube::new(
                    mq_ctx,
                    material::SDF_CIRCLE,
                )))
                .build(),
            _test_obj: Object::new(mq_ctx, material::DEBUG_COLOR)
                .position(Vec3::new(-1.0, 0.0, 0.0))
                .build(),
            windows: windows::Windows::new(egui_mq.egui_ctx()),
            modul: modul::Modul::new(config),
            egui_mq,
            rotation: 0.0,
        }
    }
}

impl mq::EventHandler for Stage {
    fn update(&mut self, _ctx: &mut mq::Context) {
        self.modul.update();

        // Update visual objects
        // self.some_obj.update() looks nicer
        self.rotation += 0.01;

        for i in 0..self.quads.len() {
            self.quads[i].transform.rotation =
                Quat::from_euler(EulerRot::XYZ, 0.0, self.rotation, 0.0);
        }
    }

    fn draw(&mut self, ctx: &mut mq::Context) {
        let (width, height) = ctx.screen_size();
        let proj = Mat4::perspective_rh_gl(60.0f32.to_radians(), width / height, 0.01, 10.0);
        let view = Mat4::look_at_rh(
            vec3(0.0, 0.0, 3.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );
        let view_proj = proj * view;

        ctx.begin_default_pass(mq::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));

        // Draw things behind egui here

        // All quads share the same vertices

        ctx.apply_pipeline(self.quads[0].get_pipeline());
        ctx.apply_bindings(self.quads[0].get_bindings());
        for i in 0..self.quads.len() {
            let model = Mat4::from_scale_rotation_translation(
                self.quads[i].transform.scale,
                self.quads[i].transform.rotation,
                self.quads[i].transform.position,
            );
            ctx.apply_uniforms(&material::Uniforms {
                mvp: view_proj * model,
                wavepoint: self.modul.get_sample_averages()[i],
                text: (0, 0, 0, 0),
            });

            ctx.draw(0, 6, 1);
        }

        // Draw generic item
        /*
        let model = Mat4::from_scale_rotation_translation(
            self._test_obj.transform.scale,
            self._test_obj.transform.rotation,
            self._test_obj.transform.position,
        );
        ctx.apply_pipeline(&self._test_obj.get_pipeline());
        ctx.apply_bindings(&self._test_obj.get_bindings());
        ctx.apply_uniforms(&material::Uniforms {
            mvp: view_proj * model,
            wavepoint: self.modul.get_sample_averages()[0],
            text: (0, 0, 0, 0),
        });
        ctx.draw(0, 6, 1);
        */
        // ================

        // Play-through
        if self.modul.is_play_through() {
            // 8 is the index of the last element
            let wavepoint = self.modul.get_sample_averages()[8];
            self.rotation += if wavepoint > 0.05 {
                -wavepoint
            } else {
                wavepoint
            };

            // Draw big plane
            let text = TEXTS[(wavepoint * 1000.0) as usize % 7];
            ctx.apply_pipeline(self.big_quad.get_pipeline());
            ctx.apply_bindings(self.big_quad.get_bindings());
            ctx.apply_uniforms(&material::Uniforms {
                mvp: view_proj,
                wavepoint,
                text,
            });
            ctx.draw(0, 6, 1);
            // ============

            // Draw cube
            let model = Mat4::from_rotation_x(self.rotation) * Mat4::from_rotation_y(self.rotation);
            let text = TEXTS[(wavepoint * 1000.0) as usize % 7];
            ctx.apply_pipeline(self.cube.get_pipeline());
            ctx.apply_bindings(self.cube.get_bindings());
            ctx.apply_uniforms(&material::Uniforms {
                mvp: view_proj * model,
                wavepoint,
                text,
            });
            ctx.draw(0, 36, 1);
            // ============
        }
        // ============

        ctx.end_render_pass();

        self.egui_mq.run(ctx, |_mq_ctx, egui_ctx| {
            draw_ui(&mut self.windows, egui_ctx, &mut self.modul);
        });

        self.egui_mq.draw(ctx);

        // Draw things in front of egui here

        ctx.commit_frame();
    }

    fn mouse_motion_event(&mut self, _ctx: &mut mq::Context, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut mq::Context, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx, dy);
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

    mq::start(conf, |mut ctx| Box::new(Stage::new(&mut ctx, config)));
}

pub fn load_image(path: &Path) -> image::DynamicImage {
    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    image::open(path).expect("image not found")
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
