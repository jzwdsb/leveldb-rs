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
