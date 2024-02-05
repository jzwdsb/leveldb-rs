pub mod leveldb {
    use crate::util::level_db::decode_fixed32;

    const M: u32 = 0xc6a4a793;
    const R: u32 = 24;

    pub fn hash(data: &[u8], seed: u32) -> u32 {
        let mut h = seed ^ (data.len() as u32).wrapping_mul(M);
        let mut cursor = 0;

        while cursor + 4 <= data.len() {
            let w = decode_fixed32(&data[cursor..]);
            h = h.wrapping_add(w).wrapping_mul(M);
            h ^= h.wrapping_shr(16);
            cursor += 4;
        }

        // pick up remaining bytes
        let remains = data.len() - cursor;
        for i in 0..remains {
            h = h.wrapping_add((data[cursor + i] as u32) << (i * 8));
        }

        if remains > 0 {
            h = h.wrapping_mul(M);
            h ^= h.wrapping_shr(R);
        }

        h
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::leveldb::*;

    #[test]
    fn test_hash() {
        let data1 = [0x62];
        let data2 = [0xc3, 0x97];
        let data3 = [0xe2, 0x99, 0xa5];
        let data4 = [0xe1, 0x80, 0xb9, 0x32];
        let data5 = [
            0x01, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x14,
            0x00, 0x00, 0x00, 0x18, 0x28, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        const TEST_SEED: u32 = 0xbc9f1d34;

        assert_eq!(hash(&data1, TEST_SEED), 0xef1345c4);
        assert_eq!(hash(&data2, TEST_SEED), 0x5b663814);
        assert_eq!(hash(&data3, TEST_SEED), 0x323c078f);
        assert_eq!(hash(&data4, TEST_SEED), 0xed21633a);
        assert_eq!(hash(&data5, 0x12345678), 0xf333dabb);
    }
}
