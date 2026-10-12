use std::collections::HashMap;
use wgpu::Extent3d;
use wgpu::naga::BuiltIn::Position;
use crate::engine::back::render::selective_systems::id::{Id32, Id32Range};
use crate::engine::back::render::selective_systems::texture_atlas::TextureAtlas;

type Id = u32;
type ChunkId = u32;
type ActivatorsCount = u32;
type Index = usize;

pub struct DataRegistryArray<T> {
    pub type_: HashMap<Id, Vec<T>>,
    pub activators: HashMap<Id, ActivatorsCount>,
    pub type_i: HashMap<Id, [usize; 2]>,
    
    pub ids: Id32,
}

impl<T> DataRegistryArray<T> {
    pub fn new() -> Self {
        Self {
            type_: HashMap::new(),
            activators: HashMap::new(),
            type_i: HashMap::new(),
            ids: Id32::new(),
        }
    }
}

pub struct DataRegistry<T> {
    pub type_: HashMap<Id, Option<T>>,
    pub chunk: HashMap<Id, ChunkId>,
    pub type_i: HashMap<Id, Index>,
    pub i_type: HashMap<Index, Id>,
    
    pub ids: Id32,
}

impl<T> DataRegistry<T> {
    pub fn new() -> Self {
        Self {
            type_: HashMap::new(),
            chunk: HashMap::new(),
            type_i: HashMap::new(),
            i_type: HashMap::new(),
            ids: Id32::new(),
        }
    }
}

pub struct DataRegistryPlus<T> {
    pub type_: HashMap<Id, Option<T>>,
    pub chunk: HashMap<Id, ChunkId>,
    pub type_i: HashMap<Id, Index>,
    pub i_type: HashMap<Index, Id>,
    pub payload: HashMap<Id, Vec<Vec<u32>>>,

    pub ids: Id32,
}

impl<T> DataRegistryPlus<T> {
    pub fn new() -> Self {
        Self {
            type_: HashMap::new(),
            chunk: HashMap::new(),
            type_i: HashMap::new(),
            i_type: HashMap::new(),
            payload: HashMap::new(),
            
            ids: Id32::new(),
        }
    }
}

pub struct LidRegistry<T> {
    pub type_: Vec<T>,
    pub ids: Id32,
}

impl<T: Clone> LidRegistry<T> {
    pub fn new(default_space: usize, fake: T) -> Self {
        Self {
            type_: vec![fake; default_space],
            ids: Id32::new(),
        }
    }
}

pub struct LidRegistryArray<T> {
    pub type_: Vec<T>,
    pub ids: Id32Range,
}

impl<T: Clone> LidRegistryArray<T> {
    pub fn new(default_space: usize, fake: T) -> Self {
        Self {
            type_: vec![fake; default_space],
            ids: Id32Range::new(),
        }
    }
}

pub struct LidRegistryTexture {
    pub atlas_texture: TextureAtlas,
    pub chunks_size: usize,
}

impl LidRegistryTexture {
    pub fn new(size: Extent3d, chunks_size: usize) -> Self {
        Self {
            atlas_texture: TextureAtlas::new(size),
            chunks_size,
        }
    }
}