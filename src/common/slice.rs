#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Slice {
    data: Vec<u8>,
}

impl Slice {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn new_with_data(data: &[u8]) -> Self {
        unimplemented!()
    }

    pub fn new_with_string(data: &str) -> Self {
        unimplemented!()
    }

    pub fn start_with(&self, prefix: &Self) -> bool {
        unimplemented!()
    }
}

impl ToString for Slice {
    fn to_string(&self) -> String {
        unimplemented!()
    }
}

impl From<&str> for Slice {
    fn from(s: &str) -> Self {
        unimplemented!()
    }
}

impl Default for Slice {
    fn default() -> Self {
        unimplemented!()
    }
}
