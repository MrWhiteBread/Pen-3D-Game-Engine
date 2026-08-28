use std::collections::HashMap;
use crate::engine::back::render::selective_systems::id::{Id32, Id32Range};

pub struct DataRegistryArray<T> {
    pub type_: HashMap<u32, Vec<T>>,
    pub chunk: HashMap<u32, u32>,
    pub type_i: HashMap<u32, [usize; 2]>,
    pub ids: Id32,
}

impl<T> DataRegistryArray<T> {
    pub fn new() -> Self {
        Self {
            type_: HashMap::new(),
            chunk: HashMap::new(),
            type_i: HashMap::new(),
            ids: Id32::new(),
        }
    }
}

pub struct DataRegistry<T> {
    pub type_: HashMap<u32, Option<T>>,
    pub chunk: HashMap<u32, u32>,
    pub type_i: HashMap<u32, usize>,
    pub i_type: HashMap<usize, u32>,
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

pub struct DataRegistryPlus<T, const N: usize> {
    pub type_: HashMap<u32, (Option<T>, [u32; N])>,
    pub chunk: HashMap<u32, u32>,
    pub type_i: HashMap<u32, usize>,
    pub i_type: HashMap<usize, u32>,
    pub ids: Id32,
}

impl<T, const N: usize> DataRegistryPlus<T, N> {
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