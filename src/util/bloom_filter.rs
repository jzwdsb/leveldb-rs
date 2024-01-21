use crate::common::FilterPolicy;
use crate::common::Slice;

pub struct BloomFilterPolicy {}

impl BloomFilterPolicy {
    pub fn new(bits_per_key: usize) -> Self {
        todo!()
    }
}

impl FilterPolicy for BloomFilterPolicy {
    fn name(&self) -> &str {
        todo!()
    }

    fn create_filter(&self, keys: &Slice) -> Slice {
        todo!()
    }

    fn key_may_match(&self, key: &Slice, filter: &Slice) -> bool {
        todo!()
    }
}
