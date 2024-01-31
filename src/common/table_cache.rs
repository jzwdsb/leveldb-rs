use super::*;

pub struct TableCache {
    cache: Box<dyn Cache>,
    dbname: String,
    options: Options,
    env: Box<dyn Env>,
}

impl TableCache {
    pub fn new(dbname: String, options: Options, entries: u64) -> Self {
        unimplemented!()
    }

    pub fn new_iter(
        &mut self,
        options: &ReadOptions,
        file_number: u64,
        file_size: u64,
    ) -> Box<dyn Iter> {
        unimplemented!()
    }

    pub fn get(
        &mut self,
        options: &ReadOptions,
        file_number: u64,
        file_size: u64,
        key: &Slice,
        handle_value: &mut Box<dyn Handle>,
    ) -> Status {
        unimplemented!()
    }

    pub fn evict(&mut self, file_number: u64) {
        unimplemented!()
    }

    fn find_table(&mut self, file_number: u64, file_size: u64) -> Option<Box<dyn Handle>> {
        unimplemented!()
    }
}
