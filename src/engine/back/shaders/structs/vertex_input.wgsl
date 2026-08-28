struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) old_color: vec4<f32>,
    @location(3) old_pos: vec3<f32>,
    @location(4) normal: vec3<f32>,
    @location(5) old_normal: vec3<f32>,
    @location(6) material: vec2<f32>,
    @location(7) old_material: vec2<f32>,
    @location(8) uv: vec2<f32>,
    @location(9) old_uv: vec2<f32>,
    @location(10) object_id: u32,
    @location(11) skeleton: u32,
}