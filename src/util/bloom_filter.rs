mod leveldb {
    use crate::common::FilterPolicy;
    use crate::common::Slice;
    use crate::util::hash::leveldb::hash;

    pub struct BloomFilterPolicy {
        bit_per_key: usize,
        k: usize,
    } // struct BloomFilterPolicy

    impl BloomFilterPolicy {
        pub fn new(bits_per_key: usize) -> Self {
            Self {
                bit_per_key: bits_per_key,
                k: ((bits_per_key as f64 * 0.69) as usize).max(1).min(30), // 0.69 =~ ln(2)
            }
        }
    } // impl BloomFilterPolicy

    impl FilterPolicy for BloomFilterPolicy {
        fn name(&self) -> &'static str {
            "leveldb.BuiltinBloomFilter2"
        }

        fn create_filter(&self, keys: &[Slice]) -> Slice {
            let bits = (keys.len() * self.bit_per_key).max(64);
            let bytes = (bits + 7) / 8;
            let bits = bytes * 8;
            let mut dst = vec![0u8; bytes];
            dst[0] = self.k as u8;
            for i in 0..keys.len() {
                let mut h = bloom_hash(&keys[i]);
                let delta = h >> 17 | h << 15;
                for _ in 0..self.k {
                    let bit_pos = h % bits as u32;
                    dst[(bit_pos / 8) as usize] |= 1 << (bit_pos % 8);
                    h += delta;
                }
            }

            Slice::new_with_vec(dst)
        }

        fn key_may_match(&self, key: &Slice, filter: &Slice) -> bool {
            let len = filter.len();
            if len < 2 {
                return false;
            }
            let array = filter.data();
            let bits = (len - 1) * 8;
            let k = array[len - 1] as usize;
            if k > 30 {
                return true;
            }
            let mut h = bloom_hash(key);
            let delta = h >> 17 | h << 15;
            for _ in 0..k {
                let bit_pos = h % bits as u32;
                if (array[(bit_pos / 8) as usize] & (1 << (bit_pos % 8))) == 0 {
                    return false;
                }
                h += delta;
            }
            true
        }
    } // impl FilterPolicy for BloomFilterPolicy

    const BLOOM_HASH_SEED: u32 = 0xbc9f1d34;

    fn bloom_hash(key: &Slice) -> u32 {
        hash(key.data(), BLOOM_HASH_SEED)
    }
} // mod leveldb
