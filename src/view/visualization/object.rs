use glam::{Quat, Vec3};
use miniquad::{Bindings, Pipeline};

use super::{material, quad::Quad};
use miniquad::graphics::GraphicsContext;

pub struct Object {
    pub shape: Box<dyn Shape>,
    // shader
    pub transform: Transform,
}

impl Object {
    pub fn new(ctx: &mut GraphicsContext) -> Self {
        Self {
            shape: Box::new(Quad::new(ctx, 1.0, 1.0, material::DEBUG_COLOR)),
            transform: Transform::default(),
        }
    }
}

pub trait Shape {
    fn get_pipeline(&self) -> &Pipeline;
    fn get_bindings(&self) -> &Bindings;
}

pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}
