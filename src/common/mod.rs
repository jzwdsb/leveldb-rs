mod code;
mod options;
mod slice;

pub use slice::*;

use self::code::Status;

pub trait Handle {
    fn key(&self) -> &Slice;
    fn value(&self) -> &Slice;
    fn value_mut(&mut self) -> &mut Slice;
    fn next(&self) -> Option<Box<dyn Handle>>;
    fn prev(&self) -> Option<Box<dyn Handle>>;
    fn set_next(&mut self, next: Option<Box<dyn Handle>>);
    fn set_prev(&mut self, prev: Option<Box<dyn Handle>>);
}

pub trait Cache {
    fn insert(&mut self, key: &Slice, value: &Slice) -> Box<dyn Handle>;
    fn lookup(&mut self, key: &Slice) -> Option<Box<dyn Handle>>;
    fn release(&mut self, entry: Box<dyn Handle>);
    fn erase(&mut self, key: &Slice);
    fn new_id(&mut self) -> u64;
    fn prune(&mut self);
}

pub trait FilterPolicy {
    fn name(&self) -> &str;
    fn create_filter(&self, keys: &Slice) -> Slice;
    fn key_may_match(&self, key: &Slice, filter: &Slice) -> bool;
}

pub trait Comparator {
    fn compare(&self, a: &Slice, b: &Slice) -> std::cmp::Ordering;
    fn name(&self) -> &str;
    fn find_shortest_separator(&self, start: &mut Slice, limit: &Slice);
    fn find_short_successor(&self, key: &mut Slice);
}

pub trait Env {
    fn new_sequential_file(&self, fname: &str, dest: &mut Box<dyn SequentialFile>) -> Status;
    fn new_random_access_file(&self, fname: &str, dest: &mut Box<dyn RandomAccessFile>) -> Status;
    fn new_writeable_file(&self, fname: &str, dest: &mut Box<dyn WritableFile>) -> Status;
    fn new_appendable_file(&self, fname: &str, dest: &mut Box<dyn WritableFile>) -> Status;

    fn file_exists(&self, fname: &str) -> bool;
    fn get_children(&self, dir: &str) -> Result<Vec<String>, Status>;
    fn remove_file(&self, fname: &str) -> Status;
    #[deprecated]
    fn delete_file(&self, fname: &str) -> Status {
        self.remove_file(fname)
    }

    fn create_dir(&self, dirname: &str) -> Status;
    fn remove_dir(&self, dirname: &str) -> Status;
    #[deprecated]
    fn delete_dir(&self, dirname: &str) -> Status {
        self.remove_dir(dirname)
    }
    fn get_file_size(&self, fname: &str) -> Result<usize, Status>;
    fn rename_file(&self, src: &str, target: &str) -> Status;

    fn lock_file(&self, fname: &str, lock: &mut Box<dyn FileLock>) -> Status;
    fn unlock_file(&self, lock: &mut Box<dyn FileLock>) -> Status;

    fn schedule(&self, f: Box<dyn FnOnce() + Send>) -> Status;
    fn start_thread(&self, name: &str, f: Box<dyn FnOnce() + Send>) -> Status;

    fn get_test_directory(&self) -> String;

    fn now_micros(&self) -> u64;
    fn sleep_for_microseconds(&self, micros: u64);
}

pub trait SequentialFile {
    fn read(&mut self, n: usize) -> Slice;
    fn skip(&mut self, n: usize);
}

pub trait WritableFile {
    fn append(&mut self, data: &Slice);
    fn close(&mut self);
    fn flush(&mut self);
    fn sync(&mut self);
}

pub trait RandomAccessFile {
    fn read(&mut self, offset: usize, n: usize) -> Slice;
}

pub trait Logger {
    fn log(&mut self, msg: &str);
}

pub trait FileLock {
    fn unlock(&mut self);
}

pub trait Table {}
