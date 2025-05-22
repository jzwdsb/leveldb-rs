use crate::port::Mutex;

use crate::common::{Cache, Handle, Slice};

use std::collections::HashMap;

struct LRUHandle {
    value: Vec<u8>,
    next: Option<Box<dyn Handle>>,
    prev: Option<Box<dyn Handle>>,
    next_hash: Option<Box<dyn Handle>>,
    charge: usize,
    key_length: usize,
    in_cache: bool,
    refs: usize,
    hash: u64,
    key_data: [u8; 1],
}

impl Handle for LRUHandle {
    fn key(&self) -> &Slice {
        todo!()
    }

    fn value(&self) -> &Slice {
        todo!()
    }

    fn value_mut(&mut self) -> &mut Slice {
        todo!()
    }

    fn next(&self) -> Option<Box<dyn Handle>> {
        todo!()
    }

    fn prev(&self) -> Option<Box<dyn Handle>> {
        todo!()
    }

    fn set_next(&mut self, next: Option<Box<dyn Handle>>) {
        todo!()
    }

    fn set_prev(&mut self, prev: Option<Box<dyn Handle>>) {
        todo!()
    }
}

pub struct LRUCache {
    capacity: usize,
    size: usize,

    usage: Mutex<usize>,
    lru: Mutex<LRUHandle>,
    in_use: Mutex<LRUHandle>,
    table: Mutex<HashMap<Slice, LRUHandle>>,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        unimplemented!()
    }
}

impl Cache for LRUCache {
    fn insert(&mut self, key: &Slice, value: &Slice) -> Box<dyn Handle> {
        todo!()
    }

    fn lookup(&mut self, key: &Slice) -> Option<Box<dyn Handle>> {
        todo!()
    }

    fn release(&mut self, entry: Box<dyn Handle>) {
        todo!()
    }

    fn erase(&mut self, key: &Slice) {
        todo!()
    }

    fn new_id(&mut self) -> u64 {
        todo!()
    }

    fn prune(&mut self) {
        todo!()
    }
}
