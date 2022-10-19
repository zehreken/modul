use std::path::Path;

use super::{material::*, object::Shape};
use miniquad as mq;
use miniquad::*;
use mq::{BlendState, BlendValue};

pub struct Cube {
    pipeline: Pipeline,
    bindings: Bindings,
}

impl Shape for Cube {
    fn get_pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    fn get_bindings(&self) -> &Bindings {
        &self.bindings
    }
}

impl Cube {
    pub fn new(ctx: &mut Context, scale_x: f32, scale_y: f32, fragment: &str) -> Cube {
        let image = super::super::load_image(Path::new("assets/atlas_gr.png"));
        let texture = Texture::from_rgba8(ctx, 512, 512, image.as_bytes());
        #[rustfmt::skip]
        let vertices: [Vertex; 24] = [
            // Front face
            Vertex { pos : Vec3 { x: -1.0 * scale_x, y: -1.0 * scale_y, z:  1.0}, uv: Vec2 { x: 0.0, y: 0.0 } },
            Vertex { pos : Vec3 { x:  1.0 * scale_x, y: -1.0 * scale_y, z:  1.0}, uv: Vec2 { x: 1.0, y: 0.0 } },
            Vertex { pos : Vec3 { x:  1.0 * scale_x, y:  1.0 * scale_y, z:  1.0}, uv: Vec2 { x: 1.0, y: 1.0 } },
            Vertex { pos : Vec3 { x: -1.0 * scale_x, y:  1.0 * scale_y, z:  1.0}, uv: Vec2 { x: 0.0, y: 1.0 } },
            // Back face
            Vertex { pos : Vec3 { x: -1.0 * scale_x, y: -1.0 * scale_y, z: -1.0}, uv: Vec2 { x: 0.0, y: 0.0 } },
            Vertex { pos : Vec3 { x: -1.0 * scale_x, y:  1.0 * scale_y, z: -1.0}, uv: Vec2 { x: 1.0, y: 0.0 } },
            Vertex { pos : Vec3 { x:  1.0 * scale_x, y:  1.0 * scale_y, z: -1.0}, uv: Vec2 { x: 1.0, y: 1.0 } },
            Vertex { pos : Vec3 { x:  1.0 * scale_x, y: -1.0 * scale_y, z: -1.0}, uv: Vec2 { x: 0.0, y: 1.0 } },
            // Top face
            Vertex { pos : Vec3 { x: -1.0 * scale_x, y:  1.0 * scale_y, z: -1.0}, uv: Vec2 { x: 0.0, y: 0.0 } },
            Vertex { pos : Vec3 { x: -1.0 * scale_x, y:  1.0 * scale_y, z:  1.0}, uv: Vec2 { x: 1.0, y: 0.0 } },
            Vertex { pos : Vec3 { x:  1.0 * scale_x, y:  1.0 * scale_y, z:  1.0}, uv: Vec2 { x: 1.0, y: 1.0 } },
            Vertex { pos : Vec3 { x:  1.0 * scale_x, y:  1.0 * scale_y, z: -1.0}, uv: Vec2 { x: 0.0, y: 1.0 } },
            // Bottom face
            Vertex { pos : Vec3 { x: -1.0 * scale_x, y: -1.0 * scale_y, z: -1.0}, uv: Vec2 { x: 0.0, y: 0.0 } },
            Vertex { pos : Vec3 { x:  1.0 * scale_x, y: -1.0 * scale_y, z: -1.0}, uv: Vec2 { x: 1.0, y: 0.0 } },
            Vertex { pos : Vec3 { x:  1.0 * scale_x, y: -1.0 * scale_y, z:  1.0}, uv: Vec2 { x: 1.0, y: 1.0 } },
            Vertex { pos : Vec3 { x: -1.0 * scale_x, y: -1.0 * scale_y, z:  1.0}, uv: Vec2 { x: 0.0, y: 1.0 } },
            // Right face
            Vertex { pos : Vec3 { x:  1.0 * scale_x, y: -1.0 * scale_y, z: -1.0}, uv: Vec2 { x: 0.0, y: 0.0 } },
            Vertex { pos : Vec3 { x:  1.0 * scale_x, y:  1.0 * scale_y, z: -1.0}, uv: Vec2 { x: 1.0, y: 0.0 } },
            Vertex { pos : Vec3 { x:  1.0 * scale_x, y:  1.0 * scale_y, z:  1.0}, uv: Vec2 { x: 1.0, y: 1.0 } },
            Vertex { pos : Vec3 { x:  1.0 * scale_x, y: -1.0 * scale_y, z:  1.0}, uv: Vec2 { x: 0.0, y: 1.0 } },
            // Left face
            Vertex { pos : Vec3 { x: -1.0 * scale_x, y: -1.0 * scale_y, z: -1.0}, uv: Vec2 { x: 0.0, y: 0.0 } },
            Vertex { pos : Vec3 { x: -1.0 * scale_x, y: -1.0 * scale_y, z:  1.0}, uv: Vec2 { x: 1.0, y: 0.0 } },
            Vertex { pos : Vec3 { x: -1.0 * scale_x, y:  1.0 * scale_y, z:  1.0}, uv: Vec2 { x: 1.0, y: 1.0 } },
            Vertex { pos : Vec3 { x: -1.0 * scale_x, y:  1.0 * scale_y, z: -1.0}, uv: Vec2 { x: 0.0, y: 1.0 } },
        ];

        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        #[rustfmt::skip]
        let indices: [u16; 36] = [
            0, 1, 2, 0, 2, 3, // front
            4, 5, 6, 4, 6, 7, // back
            8, 9, 10, 8, 10, 11, // top
            12, 13, 14, 12, 14, 15, // bottom
            16, 17, 18, 16, 18, 19, // right
            20, 21, 22, 20, 22, 23, // left
        ];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        let shader = Shader::new(ctx, VERTEX, fragment, text_shader_meta()).unwrap();
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
                depth_test: Comparison::LessOrEqual,
                depth_write: true,
                color_blend: Some(color_blend),
                alpha_blend: Some(alpha_blend),
                ..Default::default()
            },
        );

        Cube { pipeline, bindings }
    }
}
