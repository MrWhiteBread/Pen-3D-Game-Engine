#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightType {
    pub position: [f32; 4],
    pub lerp_position: [f32; 4],
    pub old_position: [f32; 4],

    pub rotation: [f32; 4],
    pub lerp_rotation: [f32; 4],
    pub old_rotation: [f32; 4],

    pub color: [f32; 4],
    pub lerp_color: [f32; 4],
    pub old_color: [f32; 4],

    pub attributes: [f32; 4],
    pub lerp_attributes: [f32; 4],
    pub old_attributes: [f32; 4],

    pub attributes2: [f32; 4],
    pub lerp_attributes2: [f32; 4],
    pub old_attributes2: [f32; 4],

    pub attributes3: [f32; 4],
    pub lerp_attributes3: [f32; 4],
    pub old_attributes3: [f32; 4],

    pub attributes4: [f32; 4],
    pub lerp_attributes4: [f32; 4],
    pub old_attributes4: [f32; 4],
    
    pub screen_position: [f32; 4],
}

impl LightType {
    pub fn new(position: [f32; 3], rotation: [f32; 3], color: [f32; 4], attributes: [f32; 4]) -> Self {
        let position = [position[0], position[1], position[2], 1.0];
        let color = [color[0], color[1], color[2], color[3]];
        let attributes = [attributes[0], attributes[1], attributes[2], 0.0];
        let rotation = [rotation[0], rotation[1], rotation[2], 1.0];

        Self {
            position,
            lerp_position: [0.0; 4],
            old_position: position,

            rotation,
            lerp_rotation: [0.0; 4],
            old_rotation: rotation,

            color,
            lerp_color: [0.0; 4],
            old_color: color,

            attributes,
            lerp_attributes: [0.0; 4],
            old_attributes: attributes,

            attributes2: [0.0, 1.0, 1.0, 0.0],
            lerp_attributes2: [0.0; 4],
            old_attributes2: [0.0, 1.0, 1.0, 0.0],

            attributes3: [1.0, 1.0, 0.0, 0.0],
            lerp_attributes3: [0.0; 4],
            old_attributes3: [1.0, 1.0, 0.0, 0.0],

            attributes4: [0.0, 0.0, 10.0, 10.0],
            lerp_attributes4: [0.0; 4],
            old_attributes4: [0.0, 0.0, 10.0, 10.0],
            
            screen_position: [0.0; 4],
        }
    }
}