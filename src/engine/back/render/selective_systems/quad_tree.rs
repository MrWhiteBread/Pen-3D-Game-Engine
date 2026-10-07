use std::alloc::{alloc, Layout};
use std::collections::HashSet;
use std::ptr::{null_mut};

pub struct QuadTree<T> {
    len: u32,
    node: *mut Node<T>,
}

impl<T: Copy> QuadTree<T> {
    pub fn new(max: u32, state: bool) -> Self {
        let max2 = max * max;
        let mut usage_hash = HashSet::with_capacity(max2 as usize);

        let nodes_layout = Layout::array::<Node<T>>(max2 as usize).expect("invalid max array size");
        let nodes = unsafe {
            alloc(nodes_layout) as *mut Node<T>
        };

        let mut n = 0;
        let node = Node::new(&[0, 0], &[max, max], &mut usage_hash, &mut n, nodes, &state);

        Self {
            len: 0,
            node,
        }
    }

    pub fn get(&mut self, value: &[u32; 2]) -> Option<&mut T> {
        unsafe {
            if self.node == null_mut() {
                return None;
            }

            (*self.node).get(value)
        }
    }

    pub fn get_cmp(&mut self, value: &[u32; 2]) -> (Option<&mut T>, [u32; 2]) {
        unsafe {
            if self.node == null_mut() {
                return (None, [0, 0]);
            }

            (*self.node).get_cmp(value)
        }
    }

    pub fn remove(&mut self, value: &[u32; 2]) {
        unsafe {
            if self.node == null_mut() {
                return;
            }

            let found = (*self.node).remove(value);

            if found {
                self.len -= 1;
            }
        }
    }

    pub fn remove_cmp(&mut self, value: &[u32; 2]) -> [u32; 2] {
        unsafe {
            if self.node == null_mut() {
                return [0; 2];
            }

            let (found, key) = (*self.node).remove_cmp(value);

            if found {
                self.len -= 1;
            }

            key
        }
    }

    pub fn insert(&mut self, key: &[u32; 2], value: T) {
        unsafe {
            if self.node == null_mut() {
                return;
            }

            let found = (*self.node).insert(key, value);

            if found {
                self.len += 1;
            }
        }
    }

    pub fn insert_cmp(&mut self, key: &[u32; 2], value: T) -> [u32; 2] {
        unsafe {
            if self.node == null_mut() {
                return [0; 2];
            }

            let (found, key) = (*self.node).insert_cmp(key, value);

            if found {
                self.len += 1;
            }

            key
        }
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }
}

struct Node<T> {
    pub used: bool,
    pub full: bool,

    pub key: [u32; 2],
    pub value: Option<T>,

    pub x1y1: *mut Node<T>,
    pub x1y0: *mut Node<T>,
    pub x0y1: *mut Node<T>,
    pub x0y0: *mut Node<T>,
}

impl<T: Copy> Node<T> {
    pub fn new(min: &[u32; 2], max: &[u32; 2], usage_hash: &mut HashSet<[u32; 2]>, n: &mut u32, nodes: *mut Node<T>, state: &bool) -> *mut Self {
        let middle = [
            min[0] + (max[0] - min[0]) / 2,
            min[1] + (max[1] - min[1]) / 2
        ];

        if usage_hash.contains(&middle) {
            return null_mut();
        }
        usage_hash.insert(middle);

        unsafe {
            let node = nodes.add(*n as usize);
            *n += 1;

            node.write(Self {
                used: *state,
                full: *state,

                key: middle,
                value: None,

                x1y1: null_mut(),
                x1y0: null_mut(),
                x0y1: null_mut(),
                x0y0: null_mut(),
            });

            if middle[0] < max[0] && middle[1] < max[1] {
                (*node).x1y1 = Node::new(&middle, max, usage_hash, n, nodes, state);
            }

            if middle[0] < max[0] && middle[1] > min[1] {
                (*node).x1y0 = Node::new(&[middle[0], min[1]], &[max[0], middle[1]], usage_hash, n, nodes, state);
            }

            if middle[0] > min[0] && middle[1] < max[1] {
                (*node).x0y1 = Node::new(&[min[0], middle[1]], &[middle[0], max[1]], usage_hash, n, nodes, state);
            }

            if middle[0] > min[0] && middle[1] > min[1] {
                (*node).x0y0 = Node::new(&[min[0], min[1]], &[middle[0], middle[1]], usage_hash, n, nodes, state);
            }

            node
        }
    }

    fn magic(&mut self) {
        unsafe {
            if self.used
                && (self.x1y1 == null_mut() || (*self.x1y1).full)
                && (self.x1y0 == null_mut() || (*self.x1y0).full)
                && (self.x0y1 == null_mut() || (*self.x0y1).full)
                && (self.x0y0 == null_mut() || (*self.x0y0).full) {

                self.full = true;
            } else {
                self.full = false;
            }
        }
    }

    fn find<F>(&mut self, key: &[u32; 2], f: &mut F) -> bool where F: FnMut(&mut Node<T>) {
        if *key == self.key {
            f(self);
            self.magic();

            return true
        }

        unsafe {
            if self.x1y1 != null_mut()
                && key[0] >= self.key[0]
                && key[1] >= self.key[1] {

                let found = (*self.x1y1).find(key, f);
                if found {
                    self.magic();

                    return true
                }
            }

            if self.x1y0 != null_mut()
                && key[0] >= self.key[0]
                && key[1] < self.key[1] {

                let found = (*self.x1y0).find(key, f);
                if found {
                    self.magic();

                    return true
                }
            }

            if self.x0y1 != null_mut()
                && key[0] < self.key[0]
                && key[1] >= self.key[1] {

                let found = (*self.x0y1).find(key, f);
                if found {
                    self.magic();

                    return true
                }
            }

            if self.x0y0 != null_mut()
                && key[0] < self.key[0]
                && key[1] < self.key[1] {

                let found = (*self.x0y0).find(key, f);
                if found {
                    self.magic();

                    return true
                }
            }

            false
        }
    }

    pub fn get(&mut self, value: &[u32; 2]) -> Option<&mut T> {
        let mut t = null_mut();
        self.find(value, &mut |node| {
            t = &mut (*node).value as *mut Option<T>;
        });

        if t != null_mut() {
            unsafe {
                (*t).as_mut()
            }
        } else {
            None
        }
    }

    pub fn remove(&mut self, value: &[u32; 2]) -> bool {
        self.find(value, &mut |node| {
            node.value = None;
            node.used = false;
        })
    }

    pub fn insert(&mut self, key: &[u32; 2], value: T) -> bool {
        self.find(key, &mut |node| {
            node.value = Some(value);
            node.used = true;
        })
    }


    pub fn find_cmp<F>(&mut self, key: &[u32; 2], f: &mut F) -> bool where F: FnMut(&mut Node<T>) {
        if key[0] <= self.key[0]
            && key[1] <= self.key[1]
            && !self.used {

            f(self);
            self.magic();

            return true
        }

        unsafe {
            if self.x1y1 != null_mut()
                && !(*self.x1y1).full {

                let found = (*self.x1y1).find_cmp(key, f);
                if found {
                    self.magic();

                    return true
                }
            }

            if self.x1y0 != null_mut()
                && key[1] < self.key[1]
                && !(*self.x1y0).full {

                let found = (*self.x1y0).find_cmp(key, f);
                if found {
                    self.magic();

                    return true
                }
            }

            if self.x0y1 != null_mut()
                && key[0] < self.key[0]
                && !(*self.x0y1).full {

                let found = (*self.x0y1).find_cmp(key, f);
                if found {
                    self.magic();

                    return true
                }
            }

            if self.x0y0 != null_mut()
                && key[0] < self.key[0]
                && key[1] < self.key[1]
                && !(*self.x0y0).full {

                let found = (*self.x0y0).find_cmp(key, f);
                if found {
                    self.magic();

                    return true
                }
            }

            false
        }
    }

    pub fn get_cmp(&mut self, key: &[u32; 2]) -> (Option<&mut T>, [u32; 2]) {
        let mut t = null_mut();
        let temp_key = [0; 2];

        self.find_cmp(key, &mut |node| {
            t = &mut (*node).value as *mut Option<T>;
        });

        if t != null_mut() {
            unsafe {
                ((*t).as_mut(), temp_key)
            }
        } else {
            (None, temp_key)
        }
    }

    pub fn remove_cmp(&mut self, key: &[u32; 2]) -> (bool, [u32; 2]) {
        let mut temp_key = [0; 2];

        let found = self.find_cmp(key, &mut |node| {
            temp_key = node.key;

            node.value = None;
            node.used = false;
        });

        (found, temp_key)
    }

    pub fn insert_cmp(&mut self, key: &[u32; 2], value: T) -> (bool, [u32; 2]) {
        let mut temp_key = [0; 2];

        let found = self.find(key, &mut |node| {
            temp_key = node.key;

            node.value = Some(value);
            node.used = true;
        });

        (found, temp_key)
    }
}