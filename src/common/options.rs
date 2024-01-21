use super::{Cache, Comparator, Env, FilterPolicy, Logger};

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
