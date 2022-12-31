use super::Transform;
use glam::{vec3, Mat4};

pub struct Camera {
    pub screen_size: (f32, f32),
    pub fov: f32,
    pub transform: Transform,
    pub elapsed_time: f32,
}

impl Camera {
    pub fn new(screen_size: (f32, f32), fov: f32) -> Self {
        Camera {
            screen_size,
            fov,
            transform: Transform::default(),
            elapsed_time: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32, wavepoint: f32) {
        self.elapsed_time += delta_time / 10.0;
        self.transform.position.x = 4.0 * self.elapsed_time.cos();
        self.transform.position.z = 4.0 * self.elapsed_time.sin();
    }

    pub fn get_view_projection(&self) -> Mat4 {
        let (width, height) = self.screen_size;
        let proj = Mat4::perspective_rh_gl(self.fov.to_radians(), width / height, 0.01, 10.0);
        let view = Mat4::look_at_rh(
            self.transform.position,
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );

        proj * view
    }
}
