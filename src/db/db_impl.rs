pub mod level_db {

    use crate::common::{
        Comparator, Env, Options, ReadOptions, Slice, Status, TableCache, WriteBatch, WriteOptions,
        DB,
    };

    struct ManualCompaction {
        level: i32,
        done: bool,
        begin: Slice,
        end: Slice,
    }

    struct CompactionStats {
        micros: u64,
        bytes_read: u64,
        bytes_written: u64,
    }

    pub struct DBImpl {
        env: Box<dyn Env>,
        internal_comparator: Box<dyn Comparator>,
        options: Options,
        owns_info_log: bool,
        owns_cache: bool,
        dbname: String,
        table_cache: TableCache,
    }

    impl DB for DBImpl {
        fn open(options: &Options, name: &str, db: &mut Self) -> Status {
            todo!()
        }

        fn put(&mut self, options: &WriteOptions, key: &Slice, value: &Slice) -> Status {
            todo!()
        }

        fn delete(&mut self, options: &WriteOptions, key: &Slice) -> Status {
            todo!()
        }

        fn write(&mut self, options: &WriteOptions, updates: &mut WriteBatch) -> Status {
            todo!()
        }

        fn get(&mut self, options: &ReadOptions, key: &Slice) -> Result<Slice, Status> {
            todo!()
        }

        fn new_iterator(&mut self, options: &ReadOptions) -> Box<dyn crate::common::Iter> {
            todo!()
        }

        fn get_snapshot(&mut self) -> Box<dyn crate::common::Snapshot> {
            todo!()
        }

        fn get_property(&mut self, propname: &Slice) -> Option<String> {
            todo!()
        }

        fn get_approximate_sizes(&mut self, ranges: &crate::common::Range, n: i64) -> Vec<u64> {
            todo!()
        }

        fn compact_range(&mut self, start: &Slice, end: &Slice) {
            todo!()
        }

        fn destory_db(options: &Options, name: &str) -> Status {
            todo!()
        }

        fn repair_db(options: &Options, name: &str) -> Status {
            todo!()
        }
    } // impl DB for DBImpl
} // pub mod level_db
