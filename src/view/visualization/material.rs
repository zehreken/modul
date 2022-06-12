use miniquad::*;

pub const VERTEX: &str = include_str!("shaders/vertex.vert");
pub const SDF_CIRCLE: &str = include_str!("shaders/sdf_circle.frag");
pub const SDF_BOX: &str = include_str!("shaders/sdf_box.frag");
pub const COLOR_BAR: &str = include_str!("shaders/color_bar.frag");
pub const TEXTURE: &str = include_str!("shaders/texture.frag");

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("offset", UniformType::Float2),
                UniformDesc::new("wavepoint", UniformType::Float1),
                UniformDesc::new("text", UniformType::Int4),
            ],
        },
        images: vec!["tex".to_string()],
    }
}

#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
pub struct Vertex {
    pub pos: Vec2,
    pub uv: Vec2,
}

#[repr(C)]
pub struct Uniforms {
    pub offset: (f32, f32),
    pub wavepoint: f32,
    pub text: (i32, i32, i32, i32),
}
