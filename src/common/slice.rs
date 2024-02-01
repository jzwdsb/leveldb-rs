use bytes::Bytes;
use std::ops::Index;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Slice {
    data: Bytes,
}

impl Slice {
    pub fn new() -> Self {
        Self { data: Bytes::new() }
    }

    pub fn new_with_data(data: &[u8]) -> Self {
        Self {
            data: Bytes::copy_from_slice(data),
        }
    }

    pub fn new_with_string(data: &str) -> Self {
        Self {
            data: Bytes::copy_from_slice(data.as_bytes()),
        }
    }

    pub fn start_with(&self, prefix: &Self) -> bool {
        self.data.starts_with(&prefix.data)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Index<usize> for Slice {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl ToString for Slice {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.data).to_string()
    }
}

impl From<&str> for Slice {
    fn from(s: &str) -> Self {
        Self::new_with_string(s)
    }
}

impl Default for Slice {
    fn default() -> Self {
        Self::new()
    }
}

mod test {
    use super::*;

    #[test]
    fn test_slice() {
        let s = Slice::new_with_string("hello");
        assert_eq!(s.to_string(), "hello");
    }
}
