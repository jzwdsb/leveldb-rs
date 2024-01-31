use super::DBImpl;

use crate::common::{Iter, Status};

enum Direction {
    Forward,
    Reverse,
}

pub struct DBIter<'a> {
    db: &'a DBImpl,
    // iter: DBIter<'a>,
    // direction: Direction,
    // sequence: u64,
    // status: Status,
    // saved_key: String,
    // saved_value: String,
    // valid: bool,
}

impl DBIter<'_> {
    pub fn new(db: &DBImpl, iter: DBIter<'_>) -> Self {
        // Self { db, iter }
        todo!()
    }
}

impl Iter for DBIter<'_> {
    fn seek_to_first(&mut self) {
        todo!()
    }

    fn seek_to_last(&mut self) {
        todo!()
    }

    fn seek(&mut self, target: &crate::common::Slice) {
        todo!()
    }

    fn next(&mut self) {
        todo!()
    }

    fn prev(&mut self) {
        todo!()
    }

    fn valid(&self) -> bool {
        todo!()
    }

    fn key(&self) -> &crate::common::Slice {
        todo!()
    }

    fn value(&self) -> &crate::common::Slice {
        todo!()
    }

    fn status(&mut self) -> crate::common::Status {
        todo!()
    }
}
