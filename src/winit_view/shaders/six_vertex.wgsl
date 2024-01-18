struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let bound: f32 = 1.0;
    var x = 0.0;
    var y = 0.0;
    // 0--2/ 5
    // |  /  |
    // 1 /3--4
    switch in_vertex_index {
        case 0u: {
            x = -bound;
            y = bound;
        }
        case 1u, 3u: {
            x = -bound;
            y = -bound;
        }
        case 2u, 5u: {
            x = bound;
            y = bound;
        }
        case 4u: {
            x = bound;
            y = -bound;
        }
        default: {}
    }

    out.position = vec4<f32>(x, y, 0.0, 1.0);
    // out.coord = vec2<f32>((x + bound) / 2.0, (y + bound) / 2.0);
    return out;
}