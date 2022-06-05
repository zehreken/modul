pub mod material;

use material::*;
use miniquad::*;
use std::path::Path;

pub struct Quad {
    pub pipeline: Pipeline,
    pub bindings: Bindings,
}

fn load_image() -> image::DynamicImage {
    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let im = image::open(&Path::new("assets/emulogic.png")).expect("image not found");
    println!("{}", im.as_bytes().len());
    im
}

impl Quad {
    pub fn new(ctx: &mut Context, scale_x: f32, scale_y: f32, fragment: &str) -> Quad {
        let image = load_image();
        let texture = Texture::from_rgba8(ctx, 338, 160, image.as_bytes());
        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -1.0 * scale_x, y: -1.0 * scale_y }, uv: Vec2 { x: 0.0, y: 0.0 } }, // bottom left
            Vertex { pos : Vec2 { x:  1.0 * scale_x, y: -1.0 * scale_y}, uv: Vec2 { x: 1.0, y: 0.0 } }, // bottom right
            Vertex { pos : Vec2 { x:  1.0* scale_x, y:  1.0 * scale_y }, uv: Vec2 { x: 1.0, y: 1.0 } }, // top right
            Vertex { pos : Vec2 { x: -1.0 * scale_x, y:  1.0 * scale_y}, uv: Vec2 { x: 0.0, y: 1.0 } }, // top left
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        let shader = Shader::new(ctx, material::VERTEX, fragment, material::meta()).unwrap();

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
