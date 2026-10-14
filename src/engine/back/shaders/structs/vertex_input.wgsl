struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) old_pos: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) old_normal: vec3<f32>,
    @location(4) uv: vec2<f32>,
    @location(5) old_uv: vec2<f32>,
    @location(6) object_id: u32,
    @location(7) skeleton: u32,
}