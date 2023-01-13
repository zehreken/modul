use super::visualization::material;
use crate::core::Modul;
use crate::core::TAPE_COUNT;
use crate::view::visualization::object::Object;
use crate::view::Camera;
use glam::{EulerRot, Mat4, Quat, Vec3};
use miniquad as mq;
use rand;

pub struct Scene3 {
    camera: Camera,
    background: Object,
    sphere: Object,
    rotation: f32,
}

impl Scene3 {
    pub fn new(mq_ctx: &mut mq::Context) -> Self {
        let mut rng = rand::thread_rng();

        let camera = Camera::new(mq_ctx.screen_size(), 60.0);

        Self {
            camera,
            background: Object::new(mq_ctx, material::BLOBS_2)
                .position(Vec3::new(0.0, 0.0, -1.0))
                .scale(Vec3::ONE * 6.0)
                .build(),
            sphere: Object::new(mq_ctx, material::SDF_CIRCLE)
                .shape(Box::new(super::visualization::Sphere::new(
                    mq_ctx,
                    material::BLOBS,
                )))
                .build(),
            rotation: 0.0,
        }
    }

    pub fn update(&mut self, modul: &Modul, delta_time: f32) {
        // self.some_obj.update() would looks nicer
        self.rotation += 0.1 * delta_time;

        self.camera.update(delta_time, 0.0);

        self.sphere.transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, self.rotation, 0.0);
    }

    pub fn draw(&mut self, ctx: &mut mq::Context, modul: &Modul) {
        let view_proj = self.camera.get_view_projection();

        let model = Mat4::from_scale_rotation_translation(
            self.background.transform.scale,
            self.background.transform.rotation,
            self.background.transform.position,
        );
        let text = material::TEXTS[0];
        ctx.apply_pipeline(self.background.get_pipeline());
        ctx.apply_bindings(self.background.get_bindings());
        ctx.apply_uniforms(&material::Uniforms {
            mvp: view_proj * model,
            wavepoint: modul.get_sample_averages()[8],
            text,
        });
        ctx.draw(0, self.background.get_num_elements(), 1);

        /*
        let model = Mat4::from_scale_rotation_translation(
            self.sphere.transform.scale,
            self.sphere.transform.rotation,
            self.sphere.transform.position,
        );
        ctx.apply_pipeline(self.sphere.get_pipeline());
        ctx.apply_bindings(self.sphere.get_bindings());
        ctx.apply_uniforms(&material::Uniforms {
            mvp: view_proj * model,
            wavepoint: modul.get_sample_averages()[8],
            text: (0, 0, 0, 0),
        });
        ctx.draw(0, self.sphere.get_num_elements(), 1);
        */
    }

    pub fn resize(&mut self, screen_size: (f32, f32)) {
        self.camera.screen_size = screen_size;
    }
}
