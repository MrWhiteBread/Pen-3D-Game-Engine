#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightType {
    pub position: [f32; 4],
    pub old_position: [f32; 4],

    pub rotation: [f32; 4],
    pub old_rotation: [f32; 4],


    pub color: [f32; 4],
    pub old_color: [f32; 4],

    pub attributes: [f32; 4],
    pub old_attributes: [f32; 4],
}

impl LightType {
    pub fn new(position: [f32; 3], rotation: [f32; 3], color: [f32; 4], attributes: [f32; 4]) -> Self {
        let position = [position[0], position[1], position[2], 1.0];
        let color = [color[0], color[1], color[2], color[3]];
        let attributes = [attributes[0], attributes[1], attributes[2], 0.0];
        let rotation = [rotation[0], rotation[1], rotation[2], 1.0];

        Self {
            position,
            old_position: position,

            rotation,
            old_rotation: rotation,

            color,
            old_color: color,

            attributes,
            old_attributes: attributes,
        }
    }
}