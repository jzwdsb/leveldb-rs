use crate::common::{Env, Status};

#[cfg(target_os = "windows")]
struct EnvWindows {}

#[cfg(target_os = "windows")]
impl EnvWindows {
    fn new() -> EnvWindows {
        EnvWindows {}
    }
}

#[cfg(target_family = "unix")]
struct EnvPosix {}

#[cfg(target_family = "unix")]
impl EnvPosix {
    fn new() -> EnvPosix {
        EnvPosix {}
    }
}

pub fn write_string_to_file_sync(env: Box<dyn Env>, filename: &String, data: &String) -> Status {
    todo!()
}

fn do_write_string_to_file(env: Box<dyn Env>, filename: &String, data: &String) -> Status {
    todo!("")
}
