struct VertexInput {
    @location(0) pos: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.position = vec4<f32>(in.pos, 0.0, 1.0);
    out.uv = vec2<f32>((in.pos.x + 1.0) / 2.0, (1.0 - in.pos.y) / 2.0);
    return out;
}
