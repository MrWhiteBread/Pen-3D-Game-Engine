struct Light { //TODO: MAKE LIGHT IN 2 structs  normal + old and lerp and screen_pos in lerp basically max 16 f32 per struct
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
    
    attributes2: vec4<f32>,
    lerp_attributes2: vec4<f32>,
    old_attributes2: vec4<f32>,
    
    attributes3: vec4<f32>,
    lerp_attributes3: vec4<f32>,
    old_attributes3: vec4<f32>,
    
    attributes4: vec4<f32>,
    lerp_attributes4: vec4<f32>,
    old_attributes4: vec4<f32>,
    
    screen_position: vec4<f32>,
}