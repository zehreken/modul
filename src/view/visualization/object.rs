use glam::{Quat, Vec3};
use miniquad::{Bindings, Pipeline};

use super::quad::Quad;
use miniquad::graphics::GraphicsContext;

pub struct Object {
    pub shape: Box<dyn Shape>,
    pub transform: Transform,
}

impl Object {
    pub fn new(ctx: &mut GraphicsContext, shader: &str) -> ObjectBuilder {
        ObjectBuilder::new(ctx, shader)
    }

    pub fn update() {}

    // pub fn get_pipeline()
    // pub fn get_bindings()
}

pub struct ObjectBuilder {
    shape: Box<dyn Shape>,
    transform: Transform,
}

impl ObjectBuilder {
    pub fn new(ctx: &mut GraphicsContext, shader: &str) -> Self {
        Self {
            shape: Box::new(Quad::new(ctx, 1.0, 1.0, shader)),
            transform: Transform::default(),
        }
    }

    pub fn shape(mut self, shape: Box<dyn Shape>) -> Self {
        self.shape = shape;
        self
    }

    // pub fn shader(mut self, shader: &str) -> Self {
    //     self.shader = shader.to_owned();
    //     self
    // }

    pub fn position(mut self, position: Vec3) -> Self {
        self.transform.position = position;
        self
    }

    pub fn rotation(mut self, rotation: Quat) -> Self {
        self.transform.rotation = rotation;
        self
    }

    pub fn scale(mut self, scale: Vec3) -> Self {
        self.transform.scale = scale;
        self
    }

    pub fn build(self) -> Object {
        Object {
            shape: self.shape,
            // shader: self.shader,
            transform: self.transform,
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
