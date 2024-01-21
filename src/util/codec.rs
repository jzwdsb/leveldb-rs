use crate::common::Slice;

pub fn PutFixed32(dst: &mut [u8], value: u32) {
    unimplemented!("")
}

pub fn PutFixed64(dst: &mut [u8], value: u64) {
    unimplemented!()
}

pub fn PutVarint32(dst: &mut [u8], value: u32) {
    unimplemented!()
}

pub fn PutVarint64(dst: &mut [u8], value: u64) {
    unimplemented!()
}

pub fn PutLengthPrefixedSlice(dst: &mut [u8], value: &[u8]) {
    unimplemented!()
}

pub fn GetFixed32(src: &Slice) -> u32 {
    unimplemented!()
}

pub fn GetVarint32(src: &Slice) -> Option<u32> {
    unimplemented!()
}

pub fn GetFixed64(src: &Slice) -> Option<u64> {
    unimplemented!()
}

pub fn GetLengthPrefixedSlice(src: &Slice) -> Option<&Slice> {
    unimplemented!()
}
