use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct VertexColor {
    position: [f32; 3],
    color: [f32; 3],
}

pub const VERTICES_COLOR: &[VertexColor] = &[
    VertexColor {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    VertexColor {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    VertexColor {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
}
// This will probably go into quad.rs
// 0--2/ 5
// |  /  |
// 1 /3--4
pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, 0.5, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.0],
    },
];
