use self::super::visualization::object::Object;
use super::visualization::material;
use crate::core::Modul;
use crate::view::Camera;
use glam::{vec3, EulerRot, Mat4, Quat, Vec3};
use miniquad as mq;

pub struct Scene2 {
    camera: Camera,
    objects: Vec<Object>,
    rotation: f32,
}

impl Scene2 {
    pub fn new(mq_ctx: &mut mq::Context) -> Self {
        let camera = Camera::new(mq_ctx.screen_size(), 60.0);

        let quad = Object::new(mq_ctx, material::DEBUG_COLOR)
            .position(Vec3::new(-1.0, 0.0, 0.0))
            .scale(Vec3::new(0.5, 0.5, 0.5))
            .build();

        let cube = Object::new(mq_ctx, material::DEBUG_COLOR)
            .shape(Box::new(super::visualization::cube::Cube::new(
                mq_ctx,
                material::DEBUG_COLOR,
            )))
            .scale(Vec3::new(0.5, 0.5, 0.5))
            .build();

        let sphere = Object::new(mq_ctx, material::DEBUG_COLOR)
            .shape(Box::new(super::visualization::Sphere::new(
                mq_ctx,
                material::DEBUG_COLOR,
            )))
            .position(Vec3::new(1.0, 0.0, 0.0))
            .scale(Vec3::new(0.5, 0.5, 0.5))
            .build();

        println!(
            "{}, {}, {}",
            quad.get_num_elements(),
            cube.get_num_elements(),
            sphere.get_num_elements()
        );

        let objects = vec![quad, cube, sphere];

        Self {
            camera,
            objects,
            rotation: 0.0,
        }
    }

    pub fn update(&mut self, modul: &Modul, delta_time: f32) {
        self.rotation += 0.1 * delta_time;

        self.camera.update(delta_time, 0.0);
        for obj in &mut self.objects {
            obj.transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, self.rotation, 0.0);
        }
    }

    pub fn draw(&mut self, ctx: &mut mq::Context, modul: &Modul) {
        let view_proj = self.camera.get_view_projection();

        for obj in &self.objects {
            ctx.apply_pipeline(obj.get_pipeline());
            ctx.apply_bindings(obj.get_bindings());
            let model = Mat4::from_scale_rotation_translation(
                obj.transform.scale,
                obj.transform.rotation,
                obj.transform.position,
            );
            ctx.apply_uniforms(&material::Uniforms {
                mvp: view_proj * model,
                wavepoint: 0.0,
                text: material::TEXTS[0],
            });

            ctx.draw(0, obj.get_num_elements(), 1);
        }
    }

    pub fn resize(&mut self, screen_size: (f32, f32)) {
        self.camera.screen_size = screen_size;
    }
}
