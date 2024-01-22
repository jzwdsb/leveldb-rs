use super::{Cache, Comparator, Env, FilterPolicy, Logger, Slice, Snapshot};

pub enum CompressionType {
    NoCompression,
    SnappyCompression,
    ZstdCompression,
}

pub struct Options {
    pub create_if_missing: bool,
    pub error_if_exists: bool,
    pub paranoid_checks: bool,
    pub env: Option<Box<dyn Env>>,
    pub comparator: Option<Box<dyn Comparator>>,
    pub cache: Option<Box<dyn Cache>>,
    pub filter_policy: Option<Box<dyn FilterPolicy>>,
    pub logger: Option<Box<dyn Logger>>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            create_if_missing: false,
            error_if_exists: false,
            paranoid_checks: false,
            env: None,
            comparator: None,
            cache: None,
            filter_policy: None,
            logger: None,
        }
    }
}

pub struct ReadOptions {
    pub verify_checksums: bool,
    pub fill_cache: bool,
    pub snapshot: Option<Box<dyn Snapshot>>,
}

pub struct WriteOptions {
    pub sync: bool,
}

pub struct WriteBatch {
    req: String,
}

pub struct WriteBatchIterator {
    batch: WriteBatch,
    offset: usize,
}

impl WriteBatch {
    pub fn new() -> Self {
        Self { req: String::new() }
    }

    pub fn put(&mut self, key: &Slice, value: &Slice) {
        unimplemented!()
    }

    pub fn delete(&mut self, key: &Slice) {
        unimplemented!()
    }

    pub fn clear(&mut self) {
        self.req.push_str("c");
    }

    pub fn approximate_size(&self) -> usize {
        unimplemented!()
    }

    pub fn iter(&self) -> WriteBatchIterator {
        unimplemented!()
    }
}
