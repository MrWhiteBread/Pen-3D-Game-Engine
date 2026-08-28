use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::ops;

#[derive(Eq)]
#[derive(PartialEq)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn clone(&self) -> Self {
        Self {
            start: self.start.clone(),
            end: self.end.clone(),
        }
    }

    fn from(range: &EndRange) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        other.start.cmp(&self.start)
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq)]
#[derive(PartialEq)]
struct EndRange {
    start: u32,
    end: u32,
}

impl EndRange {
    fn clone(&self) -> Self {
        Self {
            start: self.start.clone(),
            end: self.end.clone(),
        }
    }

    fn from(range: &Range) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

impl Ord for EndRange {
    fn cmp(&self, other: &Self) -> Ordering {
        other.end.cmp(&self.end)
    }
}

impl PartialOrd for EndRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Id32Range {
    last: u32,
    len: usize,
    un_range: BTreeSet<Range>,
    un_end_range: BTreeSet<EndRange>,
    un_range_size: BTreeMap<u32, BTreeSet<Range>>,
}

#[allow(dead_code)]
impl Id32Range {
    pub fn new() -> Self {
        Self {
            last: 0,
            len: 0,
            un_range: BTreeSet::new(),
            un_end_range: BTreeSet::new(),
            un_range_size: BTreeMap::new(),
        }
    }

    pub fn get_id(&mut self, size: u32) -> ops::Range<u32> {
        self.len += size as usize;

        let f = self.un_range_size.range_mut(size..).next();
        if f.is_some() {
            let (len, range_tree) = f.expect("no range");
            let range = range_tree.pop_last().expect("no last range");

            let len= len.clone();
            if range_tree.is_empty() {
                self.un_range_size.remove(&len);
            }

            self.un_range.remove(&range);
            self.un_end_range.remove(&EndRange::from(&range));

            let end = range.start + size;

            let new_range = Range { start: end, end: range.end };
            let size = new_range.end + 1 - new_range.start;

            if size > 0 {
                let range_tree = self.un_range_size.get_mut(&size);
                if range_tree.is_some() {
                    range_tree.expect("no range").insert(new_range.clone());
                } else {
                    self.un_range_size.insert(size, BTreeSet::from([new_range.clone()]));
                }

                self.un_end_range.insert(EndRange::from(&new_range));
                self.un_range.insert(new_range);
            }

            return range.start..end;
        }

        let range = Range {start: self.last, end: self.last + size};
        self.last = range.end;

        range.start..range.end
    }

    pub fn remove(&mut self, range: &[u32; 2]) {
        self.len -= (range[1] - range[0]) as usize;

        let mut start = range[0];
        let mut end = range[1];

        if start > 0 {
            let under_start = self.un_end_range.get(&EndRange {start: 0, end: start - 1});
            if let Some(under_start) = under_start {
                let under_start = (*under_start).clone();
                self.un_end_range.remove(&under_start);

                let range = Range::from(&under_start);
                self.un_range.remove(&range);

                let ss = range.end + 1 - range.start;
                let rg_size = self.un_range_size.get_mut(&ss).expect("no under range");
                rg_size.remove(&range);
                if rg_size.is_empty() {
                    self.un_range_size.remove(&ss);
                }

                start = under_start.start;
            }
        }

        let over_end = self.un_range.get(&Range {start: end + 1, end: 0});
        if let Some(over_end) = over_end {
            let over_end = over_end.clone();
            self.un_range.remove(&over_end);
            self.un_end_range.remove(&EndRange::from(&over_end));

            let ss = over_end.end + 1 - over_end.start;
            let rg_size = self.un_range_size.get_mut(&ss).expect("no under range");
            rg_size.remove(&over_end);
            if rg_size.is_empty() {
                self.un_range_size.remove(&ss);
            }

            end = over_end.end;
        }

        let size = end + 1 - start;
        let range = Range { start, end };
        self.un_end_range.insert(EndRange::from(&range));

        let range_size = self.un_range_size.get_mut(&size);
        if range_size.is_some() {
            let range_size = range_size.expect("no range size");
            range_size.insert(range.clone());
        } else {
            let range_size = BTreeSet::from([range.clone()]);
            self.un_range_size.insert(size, range_size);
        }

        self.un_range.insert(range);
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn size(&self) -> usize {
        self.last as usize
    }

    pub fn clear(&mut self) {
        self.last = 0;
        self.len = 0;
        self.un_range.clear();
        self.un_end_range.clear();
        self.un_range_size.clear();
    }
}

pub struct Id32 {
    ids: BTreeSet<u32>,
    unused_ids: BTreeSet<u32>,
    next_id: u32,
}

#[allow(dead_code)]
impl Id32 {
    pub fn new() -> Self {
        let mut ids = BTreeSet::new();
        ids.insert(0);

        Self {
            ids,
            unused_ids: BTreeSet::new(),
            next_id: 1,
        }
    }

    pub fn get_id(&mut self) -> u32 {
        let id = self.next_id;
        self.ids.insert(id);
        self.unused_ids.remove(&self.next_id);

        if self.unused_ids.is_empty() {
            self.next_id = self.ids.last().expect("no last id") + 1;
            return id - 1;
        }

        self.next_id = *self.unused_ids.first().expect("no first id");

        id - 1
    }

    pub fn remove(&mut self, id: &u32) {
        let id = id + 1;
        if !self.ids.contains(&id) {return;}

        self.ids.remove(&id);

        if id < self.next_id {
            self.unused_ids.insert(self.next_id);
            self.next_id = id
        }
        self.unused_ids.insert(id);
    }

    pub fn last(&self) -> u32 {
        if self.ids.len() > 1 && let Some(last) = self.ids.last() {
            *last - 1
        } else { 0 }
    }

    pub fn len(&self) -> usize {
        self.ids.len() - 1
    }

    pub fn size(&self) -> usize {
        if let Some(last) = self.ids.last() {
            *last as usize
        } else { 0 }
    }

    pub fn clear(&mut self) {
        self.ids.clear();
        self.unused_ids.clear();
        self.unused_ids.insert(0);
        self.next_id = 1;
    }
}