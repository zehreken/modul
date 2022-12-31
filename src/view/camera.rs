use super::Transform;
use glam::{vec3, Mat4, Vec3};
use rand::{rngs::ThreadRng, Rng};

pub struct Camera {
    pub screen_size: (f32, f32),
    pub fov: f32,
    pub transform: Transform,
    rnd: ThreadRng,
    next_pos: Vec3,
}

impl Camera {
    pub fn new(screen_size: (f32, f32), fov: f32) -> Self {
        Camera {
            screen_size,
            fov,
            transform: Transform::default(),
            rnd: rand::thread_rng(),
            next_pos: vec3(0.0, 0.0, 3.0),
        }
    }

    pub fn update(&mut self, delta_time: f32, wavepoint: f32) {
        // println!("{}", wavepoint);
        if wavepoint > 0.05 {
            let random_x = self.rnd.gen_range(-5..5);
            let random_y = self.rnd.gen_range(-3..3);
            let random_z = self.rnd.gen_range(3..4);
            self.next_pos = vec3(random_x as f32, random_y as f32, random_z as f32);
        }

        self.transform.position += (self.next_pos - self.transform.position) * delta_time * 10.0;
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
