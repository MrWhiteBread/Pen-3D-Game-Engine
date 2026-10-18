struct RawObject {
    rot: vec4<f32>,
    old_rot: vec4<f32>,

    center: vec4<f32>,
    old_center: vec4<f32>,

    size: vec4<f32>,
    old_size: vec4<f32>,

    color: u32,
    old_color: u32,

    id: u32,

    index_count: u32,
    first_index: u32,
    base_vertex: i32,
    radius: f32,

    padding: u32,
}