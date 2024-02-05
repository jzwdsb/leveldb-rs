pub mod level_db {
    use std::cmp::Ordering;

    use crate::{
        common::{Comparator, FilterPolicy, Slice},
        util::level_db::{decode_fixed64, put_fixed64},
    };

    mod config {
        const K_NUM_LEVELS: usize = 7;
        const K_L0_COMPACTION_TRIGGER: usize = 4;
        const K_L0_SLOWDOWN_WRITES_TRIGGER: usize = 8;
        const K_L0_STOP_WRITES_TRIGGER: usize = 12;
        const K_MAX_MEM_COMPACT_LEVEL: usize = 2;
        const K_READ_BYTES_PERIOD: usize = 1048576;
    } // mod config

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    enum ValueType {
        KTypeDeletion = 0x0,
        KTypeValue = 0x1,
    } // enum ValueType

    const K_VALUE_TYPE_FOR_SEEK: ValueType = ValueType::KTypeValue;
    type SequenceNumber = u64;
    const K_MAX_SEQUENCE_NUMBER: SequenceNumber = ((0x1u64) << 56) - 1;

    fn pack_sequence_and_type(seq: SequenceNumber, t: ValueType) -> u64 {
        assert!(seq <= K_MAX_SEQUENCE_NUMBER);
        (seq << 8) | t as u64
    }

    struct ParsedInternalKey {
        user_key: Slice,
        sequence: SequenceNumber,
        value_type: ValueType,
    } // struct ParsedInternalKey

    impl ParsedInternalKey {
        pub fn new(user_key: Slice, sequence: SequenceNumber, value_type: ValueType) -> Self {
            Self {
                user_key,
                sequence,
                value_type,
            }
        }

        pub fn debug_string(&self) -> String {
            format!(
                "{:?} @ {:?} : {:?}",
                self.user_key, self.sequence, self.value_type
            )
        }
        pub fn len(&self) -> usize {
            self.user_key.len() + 8
        }
    } // impl ParsedInternalKey

    impl Default for ParsedInternalKey {
        fn default() -> Self {
            Self {
                user_key: Slice::default(),
                sequence: 0,
                value_type: ValueType::KTypeValue,
            }
        }
    } // impl Default for ParsedInternalKey

    fn aoppend_internal_key(result: &mut String, key: &ParsedInternalKey) {
        result.push_str(&key.user_key.to_string());
        put_fixed64(
            result,
            pack_sequence_and_type(key.sequence.clone(), key.value_type.clone()),
        );
    }

    pub fn extract_user_key(internal_key: &Slice) -> Slice {
        Slice::new_with_data(&internal_key.data()[..internal_key.len() - 8])
    }

    struct InternalKeyComparator {
        user_comparator: Box<dyn Comparator>,
    } // struct InternalKeyComparator

    impl InternalKeyComparator {
        pub fn new(user_comparator: Box<dyn Comparator>) -> Self {
            Self { user_comparator }
        }
    }

    impl Comparator for InternalKeyComparator {
        fn compare(&self, a: &Slice, b: &Slice) -> Ordering {
            // order by
            //   increasing user key (according to user-supplied comparator)
            //   decreasing sequence number
            //   decreasing type (though sequence# should be enough to disambiguate)
            let mut r = self
                .user_comparator
                .compare(&extract_user_key(a), &extract_user_key(b));
            if r == Ordering::Equal {
                let anum = decode_fixed64(&a);
                let bnum = decode_fixed64(&b);
                r = anum.cmp(&bnum);
            }
            r
        }

        fn name(&self) -> &'static str {
            "leveldb.InternalKeyComparator"
        }

        fn find_shortest_separator(&self, start: &mut Slice, limit: &Slice) {
            todo!()
        }

        fn find_short_successor(&self, key: &mut Slice) {
            todo!()
        }
    } // impl Comparator for InternalKeyComparator

    struct InternalFilterPolicy {}

    impl FilterPolicy for InternalFilterPolicy {
        fn name(&self) -> &str {
            todo!()
        }

        fn create_filter(&self, keys: &[Slice]) -> Slice {
            todo!()
        }

        fn key_may_match(&self, key: &Slice, filter: &Slice) -> bool {
            todo!()
        }
    }

    // InternalKey is a key in the internal format.
    // module in this directory should keep internal keys wrapped inside
    // the InternalKey struct instead of using Slice directly.
    // So that we do not incorrectly use string comparisons instead of an Internal KeyComparator
    struct InternalKey {
        rep: Slice,
    } // struct InternalKey

    impl InternalKey {
        pub fn new(rep: Slice) -> Self {
            Self { rep }
        }

        pub fn debug_string(&self) -> String {
            format!("{:?}", self.rep)
        }

        pub fn decode_from(&mut self, s: &Slice) {
            self.rep = s.clone();
        }

        pub fn encode(&self) -> Slice {
            self.rep.clone()
        }

        pub fn user_key(&self) -> Slice {
            self.rep.clone()
        }

        pub fn clear(&mut self) {
            self.rep = Slice::default();
        }
    } // impl InternalKey

    impl Default for InternalKey {
        fn default() -> Self {
            Self {
                rep: Slice::default(),
            }
        }
    } // impl Default for InternalKey
} // pub mod level_db
