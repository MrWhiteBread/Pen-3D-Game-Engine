use half::f16;
use crate::engine::back::types::light::LightType;
use crate::engine::back::types::object::raw_object::Object;
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
};

pub const FAKE_VERTEX: Vertex = Vertex {
    position: [0.0; 3],
    normal: 0,
    uv: [f16::ZERO; 2],
};

pub const FAKE_INDEX: u32 = 0;

pub const FAKE_OBJECT: Object = Object {
    rot: [0.0; 4],
    old_rot: [0.0; 4],

    center: [0.0; 4],
    old_center: [0.0; 4],

    size: [0.0; 4],
    old_size: [0.0; 4],

    color: 0,
    old_color: 0,
    
    id: 0,

    index_count: 0,
    first_index: 0,
    base_vertex: 0,
    radius: 0.0,

    padding: 0,
};