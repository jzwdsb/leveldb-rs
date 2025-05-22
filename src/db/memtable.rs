pub mod level_db {
    use crate::{
        common::Slice,
        db::db_format::level_db::{InternalKeyComparator, LookupKey, ValueType},
    };

    struct KeyComparator<'a> {
        comparator: &'a  InternalKeyComparator,
    }

    impl<'a> KeyComparator<'a> {
        pub fn new(comparator: &'a InternalKeyComparator) -> KeyComparator<'a> {
            KeyComparator {
                comparator: comparator,
            }
        }
    }

    pub struct MemTable<'a> {
        comparator: KeyComparator<'a>,
        refs: i32,
        
    }

    impl<'a> MemTable<'a> {
        pub fn new(comparator: &'a InternalKeyComparator) -> MemTable<'a> {
            MemTable {
                comparator: KeyComparator::new(comparator),
                refs: 1,
            }
        }
        pub fn ref_memtable(&self) -> &MemTable {
            self
        }

        pub fn unref_memtable(&self) {
            todo!("unref_memtable")
        }

        pub fn add(&self, sequence: u64, value_type: ValueType, key: &Slice, value: &Slice) {
            todo!("add")
        }

        pub fn get(&self, key: &LookupKey) -> Option<Slice> {
            todo!("get")
        }
    }
}
