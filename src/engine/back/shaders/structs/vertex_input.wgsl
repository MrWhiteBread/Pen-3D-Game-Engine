struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) normals: u32,
    @location(2) uv: vec2<f32>,

    @location(3) object_id: u32,
}