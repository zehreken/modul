use self::{super::visualization::object::Object, super::windows::Windows};
use super::visualization::{self, material};
use crate::core::utils::TAPE_COUNT;
use crate::core::Modul;
use crate::Config;
use egui::Context;
use glam::{vec3, EulerRot, Mat4, Quat, Vec3};
use rand::{self, Rng};
use {egui_miniquad as egui_mq, miniquad as mq};

pub struct Scene {
    quads: Vec<Object>,
    cube: Object,
    sphere: Object,
    spheres: Vec<Object>,
    rotation: f32,
}

impl Scene {
    pub fn new(mq_ctx: &mut mq::Context) -> Self {
        let mut quads = Vec::with_capacity(TAPE_COUNT);
        let mut rng = rand::thread_rng();
        for i in 0..TAPE_COUNT {
            quads.push(
                Object::new(
                    mq_ctx,
                    // Fix later
                    if rng.gen_range(0..3) == 0 {
                        material::SDF_EYE
                    } else if rng.gen_range(0..3) == 1 {
                        material::SDF_BOX
                    } else if rng.gen_range(0..3) == 2 {
                        material::TEXTURE
                    } else {
                        material::COLOR_BAR
                    },
                )
                .position(Vec3::new(
                    -2.25 + (i % 4) as f32 * 1.5_f32,
                    -0.75_f32 + (i / 4) as f32 * 1.5_f32,
                    0.0,
                ))
                .scale(Vec3::new(0.75, 0.75, 0.75))
                .build(),
            );
        }
        let mut spheres = Vec::with_capacity(TAPE_COUNT);
        for i in 0..TAPE_COUNT {
            spheres.push(
                Object::new(mq_ctx, material::SDF_EYE)
                    .shape(Box::new(super::visualization::Sphere::new(
                        mq_ctx,
                        material::SDF_EYE_2,
                    )))
                    .position(Vec3::new(
                        -2.25 + (i % 4) as f32 * 1.5_f32,
                        -0.75_f32 + (i / 4) as f32 * 1.5_f32,
                        0.0,
                    ))
                    .rotation(Quat::from_euler(EulerRot::XYZ, 90.0, 0.0, 0.0))
                    .scale(Vec3::new(0.75, 0.75, 0.75))
                    .build(),
            );
        }
        Self {
            quads,
            // big_quad: Object::new(mq_ctx, material::_TEXTURE).build(),
            cube: Object::new(mq_ctx, material::SDF_CIRCLE)
                .shape(Box::new(super::visualization::cube::Cube::new(
                    mq_ctx,
                    material::SDF_CIRCLE,
                )))
                .build(),
            sphere: Object::new(mq_ctx, material::DEBUG_COLOR)
                .shape(Box::new(super::visualization::Sphere::new(
                    mq_ctx,
                    material::SDF_EYE_2,
                )))
                .build(),
            spheres,
            rotation: 0.0,
        }
    }

    pub fn update(&mut self) {
        // self.some_obj.update() would looks nicer
        self.rotation += 0.01;

        self.sphere.transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, self.rotation, 0.0);

        for i in 0..self.quads.len() {
            self.spheres[i].transform.rotation =
                Quat::from_euler(EulerRot::XYZ, 0.0, self.rotation, 0.0);
        }

        // for i in 0..self.quads.len() {
        //     self.quads[i].transform.rotation =
        //         Quat::from_euler(EulerRot::XYZ, 0.0, self.rotation, 0.0);
        // }
    }

    pub fn draw(&mut self, ctx: &mut mq::Context, modul: &Modul) {
        let (width, height) = ctx.screen_size();
        let proj = Mat4::perspective_rh_gl(60.0f32.to_radians(), width / height, 0.01, 10.0);
        let view = Mat4::look_at_rh(
            vec3(0.0, 0.0, 3.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );
        let view_proj = proj * view;

        // All quads share the same vertices
        /*
        for i in 0..self.quads.len() {
            let wavepoint = self.modul.get_sample_averages()[i];
            let text = TEXTS[(wavepoint * 1000.0) as usize % 7];
            ctx.apply_pipeline(self.quads[i].get_pipeline());
            ctx.apply_bindings(self.quads[i].get_bindings());
            let model = Mat4::from_scale_rotation_translation(
                self.quads[i].transform.scale,
                self.quads[i].transform.rotation,
                self.quads[i].transform.position,
            );
            ctx.apply_uniforms(&material::Uniforms {
                mvp: view_proj * model,
                wavepoint,
                text,
            });

            ctx.draw(0, 6, 1);
        }
        */

        // All spheres share the same vertices
        for i in 0..self.spheres.len() {
            let wavepoint = modul.get_sample_averages()[i];
            let text = material::TEXTS[(wavepoint * 1000.0) as usize % 7];
            ctx.apply_pipeline(self.spheres[i].get_pipeline());
            ctx.apply_bindings(self.spheres[i].get_bindings());
            let model = Mat4::from_scale_rotation_translation(
                self.spheres[i].transform.scale,
                self.spheres[i].transform.rotation,
                self.spheres[i].transform.position,
            );
            ctx.apply_uniforms(&material::Uniforms {
                mvp: view_proj * model,
                wavepoint,
                text,
            });

            ctx.draw(0, visualization::VERTEX_COUNT as i32 * 6, 1);
        }

        // Play-through
        if modul.is_play_through() {
            // 8 is the index of the last element
            let wavepoint = modul.get_sample_averages()[8];
            self.rotation += if wavepoint > 0.05 {
                -wavepoint
            } else {
                wavepoint
            };

            // Draw big plane
            /*
            let text = TEXTS[(wavepoint * 1000.0) as usize % 7];
            ctx.apply_pipeline(self.big_quad.get_pipeline());
            ctx.apply_bindings(self.big_quad.get_bindings());
            ctx.apply_uniforms(&material::Uniforms {
                mvp: view_proj,
                wavepoint,
                text,
            });
            ctx.draw(0, 6, 1);
            */
            // ============

            // Draw cube
            let model = Mat4::from_rotation_x(self.rotation) * Mat4::from_rotation_y(self.rotation);
            let text = material::TEXTS[(wavepoint * 1000.0) as usize % 7];
            ctx.apply_pipeline(self.cube.get_pipeline());
            ctx.apply_bindings(self.cube.get_bindings());
            ctx.apply_uniforms(&material::Uniforms {
                mvp: view_proj * model,
                wavepoint,
                text,
            });
            // ctx.draw(0, 36, 1);
            // ============

            // Draw sphere
            let model = Mat4::from_scale_rotation_translation(
                self.sphere.transform.scale,
                self.sphere.transform.rotation,
                self.sphere.transform.position,
            );
            ctx.apply_pipeline(&self.sphere.get_pipeline());
            ctx.apply_bindings(&self.sphere.get_bindings());
            ctx.apply_uniforms(&material::Uniforms {
                mvp: view_proj * model,
                wavepoint,
                text,
            });
            ctx.draw(0, visualization::VERTEX_COUNT as i32 * 6, 1);
            // ================
        }
        // ============
    }
}
