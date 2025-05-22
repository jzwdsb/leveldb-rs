use crate::common::Slice;

pub fn append_number_to(dest: &mut String, num: u64) {
    dest.push_str(&num.to_string());
}

pub fn append_escaped_string_to(dest: &mut String, src: &Slice) {
    for &c in src.data() {
        if c >= b' ' && c <= b'~' {
            dest.push(c as char);
        } else {
            dest.push_str(&format!("\\x{:02x}", c));
        }
    }
}

pub fn escape_string(src: &Slice) -> String {
    let mut dest = String::new();
    append_escaped_string_to(&mut dest, src);
    dest
}

pub fn comsume_deciamal_number(src: &mut Slice) -> Option<u64> {
    todo!("comsume_deciamal_number")
}
