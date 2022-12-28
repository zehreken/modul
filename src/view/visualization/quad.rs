use super::material::*;
use super::object::*;
use crate::core::utils::*;
use miniquad as mq;
use miniquad::*;
use mq::{BlendState, BlendValue};
use std::path::Path;

pub struct Quad {
    pipeline: Pipeline,
    bindings: Bindings,
}

impl Shape for Quad {
    fn get_pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    fn get_bindings(&self) -> &Bindings {
        &self.bindings
    }
}

impl Quad {
    pub fn new(ctx: &mut Context, fragment: &str) -> Quad {
        let image = load_image(Path::new("assets/atlas_gr.png"));
        let texture = Texture::from_rgba8(ctx, 512, 512, image.as_bytes());
        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec3 { x: -1.0, y: -1.0, z: 0.0}, uv: Vec2 { x: 0.0, y: 0.0 } }, // bottom left
            Vertex { pos : Vec3 { x:  1.0, y: -1.0, z: 0.0}, uv: Vec2 { x: 1.0, y: 0.0 } }, // bottom right
            Vertex { pos : Vec3 { x:  1.0, y:  1.0, z: 0.0}, uv: Vec2 { x: 1.0, y: 1.0 } }, // top right
            Vertex { pos : Vec3 { x: -1.0, y:  1.0, z: 0.0}, uv: Vec2 { x: 0.0, y: 1.0 } }, // top left
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        let shader = Shader::new(
            ctx,
            super::material::VERTEX,
            fragment,
            super::material::text_shader_meta(),
        )
        .unwrap();
        let color_blend = BlendState::new(
            mq::Equation::Add,
            mq::BlendFactor::Value(BlendValue::SourceColor),
            mq::BlendFactor::OneMinusValue(BlendValue::SourceColor),
        );
        let alpha_blend = BlendState::new(
            mq::Equation::Add,
            mq::BlendFactor::Value(BlendValue::SourceAlpha),
            mq::BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
        );
        let pipeline = Pipeline::with_params(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
            PipelineParams {
                depth_test: Comparison::Always,
                depth_write: true,
                color_blend: Some(color_blend),
                alpha_blend: Some(alpha_blend),
                ..Default::default()
            },
        );

        Quad { pipeline, bindings }
    }
}
