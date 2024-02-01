pub mod level_db {
    use crate::common::Slice;
    use std::mem::size_of;

    pub fn encode_fixed32(dst: &mut [u8], value: u32) {
        dst[0] = (value & 0xff) as u8;
        dst[1] = ((value >> 8) & 0xff) as u8;
        dst[2] = ((value >> 16) & 0xff) as u8;
        dst[3] = ((value >> 24) & 0xff) as u8;
    }

    pub fn encode_fixed64(dst: &mut [u8], value: u64) {
        dst[0] = (value & 0xff) as u8;
        dst[1] = ((value >> 8) & 0xff) as u8;
        dst[2] = ((value >> 16) & 0xff) as u8;
        dst[3] = ((value >> 24) & 0xff) as u8;
        dst[4] = ((value >> 32) & 0xff) as u8;
        dst[5] = ((value >> 40) & 0xff) as u8;
        dst[6] = ((value >> 48) & 0xff) as u8;
        dst[7] = ((value >> 56) & 0xff) as u8;
    }

    pub fn put_fixed32(dst: &mut String, value: u32) {
        let mut buf = [0u8; size_of::<u32>()];
        encode_fixed32(&mut buf, value);
        dst.push_str(std::str::from_utf8(&buf).unwrap());
    }

    pub fn put_fixed64(dst: &mut String, value: u64) {
        let mut buf = [0u8; size_of::<u64>()];
        encode_fixed64(&mut buf, value);
        dst.push_str(std::str::from_utf8(&buf).unwrap());
    }

    pub fn decode_fixed32(src: &Slice) -> u32 {
        (src[0] as u32) | ((src[1] as u32) << 8) | ((src[2] as u32) << 16) | ((src[3] as u32) << 24)
    }

    pub fn decode_fixed64(src: &Slice) -> u64 {
        (src[0] as u64)
            | ((src[1] as u64) << 8)
            | ((src[2] as u64) << 16)
            | ((src[3] as u64) << 24)
            | ((src[4] as u64) << 32)
            | ((src[5] as u64) << 40)
            | ((src[6] as u64) << 48)
            | ((src[7] as u64) << 56)
    }

    pub fn put_varint32(dst: &mut [u8], value: u32) {
        unimplemented!()
    }

    pub fn put_varint64(dst: &mut [u8], value: u64) {
        unimplemented!()
    }

    pub fn put_length_prefixed_slice(dst: &mut [u8], value: &[u8]) {
        unimplemented!()
    }

    pub fn get_fixed32(src: &Slice) -> u32 {
        unimplemented!()
    }

    pub fn get_varint32(src: &Slice) -> Option<u32> {
        unimplemented!()
    }

    pub fn get_fixed64(src: &Slice) -> Option<u64> {
        unimplemented!()
    }

    pub fn get_length_prefixed_slice(src: &Slice) -> Option<&Slice> {
        unimplemented!()
    }
}
