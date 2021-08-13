use miniquad::*;

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

pub struct Quad {
    pub pipeline: Pipeline,
    pub bindings: Bindings,
}

impl Quad {
    pub fn new(ctx: &mut Context) -> Quad {
        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -0.25, y: -1.0 }, uv: Vec2 { x: 0.0, y: 0.0 } }, // bottom left
            Vertex { pos : Vec2 { x:  0.25, y: -1.0 }, uv: Vec2 { x: 1.0, y: 0.0 } }, // bottom right
            Vertex { pos : Vec2 { x:  0.25, y:  1.0 }, uv: Vec2 { x: 1.0, y: 1.0 } }, // top right
            Vertex { pos : Vec2 { x: -0.25, y:  1.0 }, uv: Vec2 { x: 0.0, y: 1.0 } }, // top left
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![],
        };

        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );

        Quad { pipeline, bindings }
    }
}

pub mod shader {
    use miniquad::*;

    pub const VERTEX: &str = include_str!("shaders/vertex.vert");

    pub const FRAGMENT: &str = include_str!("shaders/fragment.frag");

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("offset", UniformType::Float2),
                    UniformDesc::new("wavepoint", UniformType::Float1),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub offset: (f32, f32),
        pub wavepoint: f32,
    }
}
