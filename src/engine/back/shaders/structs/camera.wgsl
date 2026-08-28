struct Plane {
    normal: vec3<f32>,
    d: f32,
}

struct Camera {
    view_proj: mat4x4<f32>,
    inv_view_proj: mat4x4<f32>,
    position: vec4<f32>,

    planes: array<Plane, 6>,
}