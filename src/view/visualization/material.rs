use miniquad::*;

pub const VERTEX: &str = include_str!("shaders/vertex.vert");
pub const SDF_CIRCLE: &str = include_str!("shaders/sdf_circle.frag");
pub const SDF_BOX: &str = include_str!("shaders/sdf_box.frag");
pub const COLOR_BAR: &str = include_str!("shaders/color_bar.frag");
pub const TEXTURE: &str = include_str!("shaders/texture.frag");
pub const SDF_EYE: &str = include_str!("shaders/sdf_eye.frag");
pub const DEBUG_COLOR: &str = include_str!("shaders/debug_color.frag");
pub const UV_VISUAL: &str = include_str!("shaders/uv_visual.frag");

pub fn text_shader_meta() -> ShaderMeta {
    ShaderMeta {
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("mvp", UniformType::Mat4),
                UniformDesc::new("wavepoint", UniformType::Float1),
                UniformDesc::new("text", UniformType::Int4),
            ],
        },
        images: vec!["tex".to_string()],
    }
}

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("mvp", UniformType::Mat4),
                UniformDesc::new("wavepoint", UniformType::Float1),
            ],
        },
        images: vec![],
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Vertex {
    pub pos: Vec3,
    pub uv: Vec2,
}

#[repr(C)]
pub struct Uniforms {
    pub mvp: glam::Mat4,
    pub wavepoint: f32,
    pub text: (i32, i32, i32, i32),
}
