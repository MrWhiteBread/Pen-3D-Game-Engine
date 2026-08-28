struct Object {
    rot: vec4<f32>,
    old_rot: vec4<f32>,

    center: vec4<f32>,
    lerp_center: vec4<f32>,
    old_center: vec4<f32>,

    size: vec4<f32>,
    lerp_size: vec4<f32>,
    old_size: vec4<f32>,

    status: u32,
    texture_id: u32,

    index_count: u32,
    first_index: u32,
    base_vertex: i32,
    radius: f32,

    skeleton: u32,

    padding: f32,
    rot_mat: mat3x3<f32>,
}