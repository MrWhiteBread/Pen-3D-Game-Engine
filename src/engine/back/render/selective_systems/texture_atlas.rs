use std::alloc::{alloc, dealloc, Layout};
use std::cmp::{max, Ordering};
use std::collections::{BTreeMap, HashMap, HashSet, LinkedList};
use std::ptr::{drop_in_place, hash, null_mut};
use indexmap::IndexMap;
use wgpu::{Extent3d};
use crate::engine::back::render::selective_systems::id::{Id32, Id32Range};
use crate::engine::back::render::selective_systems::quad_tree::QuadTree;

type Id = u32;
type Position = [u32; 2];
type Size = [u32; 2];

type Key = [u32; 2];

type Bounds = [u32; 4];

type Index = u32;

struct EmptySpace {
    points: HashMap<Position, HashSet<Id>>,
    rectangles: HashMap<Id, Bounds>,

    ids: Id32,
}

impl EmptySpace {
    fn new() -> Self {
        Self {
            points: HashMap::new(),
            rectangles: HashMap::new(),

            ids: Id32::new(),
        }
    }

    fn remove_points(&mut self, positions: &[Position; 4], id: &Id) {
        let mut remove_point = |pos: &Position| {
            let points = self.points.get_mut(pos).expect("HOW");
            points.remove(id);

            if points.is_empty() {
                self.points.remove(pos);
            }
        };

        remove_point(&positions[0]);
        remove_point(&positions[1]);
        remove_point(&positions[2]);
        remove_point(&positions[3]);
    }

    fn remove(&mut self, id: &Id) {
        let bounds = self.rectangles.remove(id).expect("no or yes");
        let positions = [
            [bounds[0], bounds[1]],
            [bounds[0] + bounds[2], bounds[1]],
            [bounds[0], bounds[1] + bounds[3]],
            [bounds[0] + bounds[2], bounds[1] + bounds[3]],
        ];

        self.remove_points(&positions, id);
    }

    fn insert(&mut self,  mut rectangle: [Position; 4]) -> (Id, HashMap<Size, HashSet<Id>>) {
        let mut output = (0, HashMap::new());

        fn intersect(self_: &EmptySpace, index: &usize, mut intersector: Vec<Id>, rectangle: &[Position; 4]) -> Vec<Id> {
            if let Some(points) = self_.points.get(&rectangle[*index]) {
                if intersector.is_empty() {
                    intersector.extend(points);
                } else {
                    intersector = intersector.into_iter().filter(|key| points.iter().find(|&key_| key_ == key).is_some()).collect();
                }
            }

            intersector
        }

        fn combine(self_: &mut EmptySpace, indices: [usize; 2], rectangle: &mut [Position; 4], output: &mut (Id, HashMap<Position, HashSet<Id>>)) -> bool {
            let mut intersector: Vec<Id>= Vec::new();
            intersector = intersect(self_, &indices[0], intersector, &rectangle);
            intersector = intersect(self_, &indices[1], intersector, &rectangle);

            if let Some(id) = intersector.get(0) {
                let bounds = self_.rectangles.remove(id).expect("id not found in this wierd thing..");
                let positions = [
                    [bounds[0], bounds[1]],
                    [bounds[0] + bounds[2], bounds[1]],
                    [bounds[0], bounds[1] + bounds[3]],
                    [bounds[0] + bounds[2], bounds[1] + bounds[3]],
                ];

                self_.ids.remove(id);

                if let Some(ids_hash) = (*output).1.get_mut(&bounds[2..]) {
                    ids_hash.insert(*id);

                } else {
                    let mut ids_hash = HashSet::new();
                    ids_hash.insert(*id);

                    output.1.insert([bounds[2], bounds[3]], ids_hash);
                }

                self_.remove_points(&positions, id);

                rectangle[indices[0]] = positions[indices[0]];
                rectangle[indices[1]] = positions[indices[1]];

                intersector.clear();
                true;
            }

            false
        }

        let mut working = true;

        while working {
            working = false;

            working = working || combine(self, [0, 1], &mut rectangle, &mut output);
            working = working || combine(self, [1, 2], &mut rectangle, &mut output);
            working = working || combine(self, [0, 2], &mut rectangle, &mut output);
            working = working || combine(self, [2, 3], &mut rectangle, &mut output);
        }

        let id = self.ids.get_id();
        output.0 = id;

        let mut insert_point = |index: usize| {
            if let Some(points) = self.points.get_mut(&rectangle[index]) {
                points.insert(id);
            } else {
                self.points.insert(rectangle[index], HashSet::from([id]));
            }
        };

        insert_point(0);
        insert_point(1);
        insert_point(2);
        insert_point(3);

        let bounds = [rectangle[0][0], rectangle[0][1], rectangle[1][0] - rectangle[0][0], rectangle[2][1] - rectangle[0][1]];
        self.rectangles.insert(id, bounds);

        output
    }
}

pub struct TextureAtlas {
    textures: HashMap<Id, Bounds>,
    sections: QuadTree<*mut HashSet<Id>>,
    empty_space: EmptySpace,

    ids: Id32,
}

impl TextureAtlas {
    pub fn new(size: Extent3d) -> Self { 
        let mut sections = QuadTree::new(max(size.width, size.height), true);
        sections.remove(&[size.width, size.height]);

        Self {
            textures: HashMap::new(),
            sections,
            empty_space: EmptySpace::new(),

            ids: Id32::new(),
        }
    }

    fn new_hash() -> *mut HashSet<Id> {
        unsafe {
            let layout = Layout::new::<HashSet<Id>>();

            let vector = alloc(layout) as *mut HashSet<Id>;
            vector.write(HashSet::new());

            vector
        }
    }

    fn delete_hash(hash: *mut HashSet<Id>) {
        unsafe {
            drop_in_place(hash);

            let layout = Layout::new::<HashSet<Id>>();
            dealloc(hash as *mut u8, layout);
        }
    }

    fn remove_global(&mut self, bounds: &Bounds) {
        let positions = [
            [bounds[0], bounds[1]],
            [bounds[0] + bounds[2], bounds[1]],
            [bounds[0], bounds[1] + bounds[3]],
            [bounds[0] + bounds[2], bounds[1] + bounds[3]],
        ];

        let (id, removed_ids) = self.empty_space.insert(positions);

        if let Some(hash) = self.sections.get(&[bounds[2], bounds[3]]) {
            unsafe {
                (**hash).insert(id);
            }
        } else {
            let hash = Self::new_hash();
            unsafe {
                (*hash).insert(id);
            }

            self.sections.insert(&[bounds[2], bounds[3]], hash);
        }

        for (size, ids_hash) in removed_ids {
            let ids_section_hash = self.sections.get(&size).expect("somehow error");

            for id in ids_hash {
                unsafe {
                    (**ids_section_hash).remove(&id);
                }
            }

            unsafe {
                if (**ids_section_hash).is_empty() {
                    Self::delete_hash(*ids_section_hash);

                    self.sections.remove(&size);
                }
            }
        }
    }

    pub fn remove(&mut self, id: Id) {
        self.ids.remove(&id);
        let bounds = self.textures.remove(&id).expect("bro this texture doesn t exist, take this error and cry");

        self.remove_global(&bounds);
    }

    fn insert_leftovers(&mut self, full_size: &Size, size: &Size, pos: &Position) {
        let bounds1 = [
            size[0] + pos[0],
            pos[1],
            full_size[0] - size[0],
            size[1]
        ];

        let bounds2 = [
            pos[0],
            size[1] + pos[1],
            full_size[0],
            full_size[1] - size[1],
        ];

        self.remove_global(&bounds1);
        self.remove_global(&bounds2);
    }

    pub fn insert(&mut self, size: &Size) -> Id {
        let id = self.ids.get_id();

        let (hash, full_size) = self.sections.get_cmp(size);
        let hash = hash.expect("EMPTY VRO, impossible");

        unsafe {
            let id = *(**hash).iter().next().expect("vector is empty :(");
            (**hash).remove(&id);

            let bounds = self.empty_space.rectangles.get(&id).expect("id not found in empty space").clone();

            if (**hash).is_empty() {
                self.sections.remove(&full_size);
            }

            self.empty_space.remove(&id);
            self.textures.insert(id, bounds);

            self.insert_leftovers(&full_size, size, &[bounds[0], bounds[1]]);
        }

        id
    }
}