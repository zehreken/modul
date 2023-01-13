use miniquad::*;

pub const VERTEX: &str = include_str!("shaders/vertex.vert");
pub const SDF_CIRCLE: &str = include_str!("shaders/sdf_circle.frag");
pub const SDF_BOX: &str = include_str!("shaders/sdf_box.frag");
pub const COLOR_BAR: &str = include_str!("shaders/color_bar.frag");
pub const TEXTURE: &str = include_str!("shaders/texture.frag");
pub const SDF_EYE: &str = include_str!("shaders/sdf_eye.frag");
pub const DEBUG_COLOR: &str = include_str!("shaders/debug_color.frag");
pub const UV_VISUAL: &str = include_str!("shaders/uv_visual.frag");
pub const SDF_EYE_2: &str = include_str!("shaders/sdf_eye_2.frag");
pub const BLOBS: &str = include_str!("shaders/blobs.frag");
pub const BLOBS_2: &str = include_str!("shaders/blobs_2.frag");

const BABY: (i32, i32, i32, i32) = (178, 177, 178, 169);
const ACID: (i32, i32, i32, i32) = (177, 179, 185, 180);
const BOMB: (i32, i32, i32, i32) = (178, 191, 189, 178);
const ZONE: (i32, i32, i32, i32) = (170, 191, 190, 181);
const WILD: (i32, i32, i32, i32) = (167, 185, 188, 180);
const SOUL: (i32, i32, i32, i32) = (163, 191, 165, 188);
const GTFO: (i32, i32, i32, i32) = (183, 164, 182, 191);
pub const TEXTS: [(i32, i32, i32, i32); 7] = [BABY, ACID, BOMB, ZONE, WILD, SOUL, GTFO];

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
