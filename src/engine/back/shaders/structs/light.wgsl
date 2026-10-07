struct Light {
    position: vec4<f32>,
    lerp_position: vec4<f32>,
    old_position: vec4<f32>,

    rotation: vec4<f32>,
    lerp_rotation: vec4<f32>,
    old_rotation: vec4<f32>,

    color: vec4<f32>,
    lerp_color: vec4<f32>,
    old_color: vec4<f32>,

    attributes: vec4<f32>,
    lerp_attributes: vec4<f32>,
    old_attributes: vec4<f32>,
}