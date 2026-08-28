use crate::engine::back::types::light::LightType;
use crate::engine::back::types::object::Object;
use crate::engine::back::types::vertex::Vertex;

pub const FAKE_LIGHT: LightType = LightType {
    position: [0.0; 4],
    lerp_position: [0.0; 4],
    old_position: [0.0; 4],
    
    rotation: [0.0; 4],
    lerp_rotation: [0.0; 4],
    old_rotation: [0.0; 4],
    
    color: [0.0; 4],
    lerp_color: [0.0; 4],
    old_color: [0.0; 4],
    
    attributes: [0.0; 4],
    lerp_attributes: [0.0; 4],
    old_attributes: [0.0; 4],

    attributes2: [0.0; 4],
    lerp_attributes2: [0.0; 4],
    old_attributes2: [0.0; 4],

    attributes3: [0.0; 4],
    lerp_attributes3: [0.0; 4],
    old_attributes3: [0.0; 4],

    attributes4: [0.0, 0.0, 10.0, 10.0],
    lerp_attributes4: [0.0; 4],
    old_attributes4: [0.0, 0.0, 10.0, 10.0],

    screen_position: [0.0; 4],
};

pub const FAKE_VERTEX: Vertex = Vertex {
    position: [0.0; 3],
    color: [0.0; 4],
    old_color: [0.0; 4],
    old_position: [0.0; 3],
    normal: [0.0; 3],
    old_normal: [0.0; 3],
    material: [0.0; 2],
    old_material: [0.0; 2],
    uv: [0.0; 2],
    old_uv: [0.0; 2],
    skeleton: 0,
};

pub const FAKE_INDEX: u32 = 0;

pub const FAKE_OBJECT: Object = Object {
    rot: [0.0; 4],
    old_rot: [0.0; 4],

    center: [0.0; 4],
    lerp_center: [0.0; 4],
    old_center: [0.0; 4],

    size: [1.0; 4],
    lerp_size: [0.0; 4],
    old_size: [1.0; 4],

    status: 0,
    texture_id: 0,

    index_count: 0,
    first_index: 0,
    base_vertex: 0,
    radius: 0.0,
    skeleton: 0,

    padding: 0.0,
    rot_mat: [[0.0; 4]; 3],
};