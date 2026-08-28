use std::collections::{HashSet};
use indexmap::IndexSet;
use crate::engine::back::render::selective_systems::dynamic_chunks::registry::{DataRegistry, DataRegistryArray, DataRegistryPlus, LidRegistry, LidRegistryArray};
use crate::engine::back::render::selective_systems::fakes::{FAKE_INDEX, FAKE_LIGHT, FAKE_OBJECT, FAKE_VERTEX};
use crate::engine::back::types::light::LightType;
use crate::engine::back::types::object::Object;
use crate::engine::back::types::vertex::Vertex;

pub struct Chunk {
    pub private: bool,
    pub loaded: bool,
    pub position: [f32; 3],

    pub lights: HashSet<u32>,
    pub objects: HashSet<u32>,
    pub vertices: HashSet<u32>,
    pub indices: HashSet<u32>,
}

pub struct LID {
    pub chunks: IndexSet<u32>,
    
    pub lights: LidRegistry<LightType>,
    pub objects: LidRegistry<Object>,
    pub vertices: LidRegistryArray<Vertex>,
    pub indices: LidRegistryArray<u32>,
}

impl LID {
    pub fn new(default_space: [usize; 4]) -> Self {
        Self {
            chunks: IndexSet::new(),

            lights: LidRegistry::new(default_space[0], FAKE_LIGHT),
            objects: LidRegistry::new(default_space[1], FAKE_OBJECT),
            vertices: LidRegistryArray::new(default_space[2], FAKE_VERTEX),
            indices: LidRegistryArray::new(default_space[3], FAKE_INDEX),
        }
    }
}

pub struct Data {
    pub lights: DataRegistry<LightType>,
    pub objects: DataRegistryPlus<Object, 2>,
    pub vertices: DataRegistryArray<Vertex>,
    pub indices: DataRegistryArray<u32>,

    pub updater: Updater,
}

impl Data {
    pub fn new() -> Self {
        Self {
            lights: DataRegistry::new(),
            objects: DataRegistryPlus::new(),
            vertices: DataRegistryArray::new(),
            indices: DataRegistryArray::new(),

            updater: Updater::new(),
        }
    }
}

pub struct Updater {
    pub lights: HashSet<u32>,
    pub objects: HashSet<u32>,
}

impl Updater {
    pub fn new() -> Self {
        Self {
            lights: HashSet::new(),
            objects: HashSet::new(),
        }
    }
}