use super::{material::*, object::Shape};
use crate::core::*;
use miniquad as mq;
use miniquad::*;
use mq::{BlendState, BlendValue};
use std::{f32::consts::PI, path::Path};

const RADIUS: f32 = 1.0;
const STACK_COUNT: usize = 8;
const STACK_STEP: f32 = PI / STACK_COUNT as f32;
const SECTOR_COUNT: usize = 12;
const SECTOR_STEP: f32 = 2_f32 * PI / SECTOR_COUNT as f32;
pub const VERTEX_COUNT: usize = (STACK_COUNT + 1) * (SECTOR_COUNT + 1);

pub struct Sphere {
    pipeline: Pipeline,
    bindings: Bindings,
}

impl Shape for Sphere {
    fn get_pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    fn get_bindings(&self) -> &Bindings {
        &self.bindings
    }
}

impl Sphere {
    pub fn new(ctx: &mut Context, fragment: &str) -> Self {
        let image = load_image(Path::new("assets/uv_mapper.png"));
        let texture = Texture::from_rgba8(ctx, 512, 512, image.as_bytes());

        let pair = get_vertices_and_indices();
        let vertices = pair.0;
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices = pair.1;
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
                depth_test: Comparison::Less,
                depth_write: true,
                color_blend: None, // Some(color_blend),
                alpha_blend: None, // Some(alpha_blend),
                ..Default::default()
            },
        );

        Sphere { pipeline, bindings }
    }
}

fn get_vertices_and_indices() -> ([Vertex; VERTEX_COUNT], [u16; VERTEX_COUNT * 6]) {
    let mut vertices = Vec::new();

    for i in 0..=STACK_COUNT {
        let stack_angle = PI / 2_f32 - i as f32 * STACK_STEP; // From PI/2 to -PI/2
        let xy = RADIUS * stack_angle.cos();
        let z = RADIUS * stack_angle.sin();

        for j in 0..=SECTOR_COUNT {
            let sector_angle = j as f32 * SECTOR_STEP; // From 0 to 2PI

            // Vertex position (x, y, z)
            let x = xy * sector_angle.cos();
            let y = xy * sector_angle.sin();

            let s = j as f32 / SECTOR_COUNT as f32;
            let t = i as f32 / STACK_COUNT as f32;
            vertices.push(Vertex {
                pos: Vec3 { x, y, z },
                uv: Vec2 { x: s, y: t },
            });
        }
    }

    let mut indices = Vec::new();
    for i in 0..STACK_COUNT {
        let mut k1: u16 = i as u16 * (SECTOR_COUNT as u16 + 1);
        let mut k2: u16 = k1 + SECTOR_COUNT as u16 + 1;

        for _ in 0..SECTOR_COUNT {
            if i != 0 {
                indices.push(k1);
                indices.push(k2);
                indices.push(k1 + 1);
            }

            if i != (STACK_COUNT - 1) {
                indices.push(k1 + 1);
                indices.push(k2);
                indices.push(k2 + 1);
            }

            k1 += 1;
            k2 += 1;
        }
    }

    let mut vertices_array: [Vertex; VERTEX_COUNT] = [Vertex::default(); VERTEX_COUNT];
    for (i, vertex) in vertices.iter().enumerate() {
        vertices_array[i] = *vertex;
    }

    let mut indices_array: [u16; VERTEX_COUNT * 6] = [0; VERTEX_COUNT * 6];
    for (i, index) in indices.iter().enumerate() {
        indices_array[i] = *index;
    }

    (vertices_array, indices_array)
}
