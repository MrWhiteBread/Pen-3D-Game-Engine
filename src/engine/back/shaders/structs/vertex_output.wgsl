struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
    @location(3) material: vec2<f32>,
    @location(4) uv: vec2<f32>,
    @location(5) @interpolate(flat) texture_id: u32,
}