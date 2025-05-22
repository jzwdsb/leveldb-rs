pub mod level_db {
    use std::sync::atomic::AtomicUsize;

    pub struct Arena {
        pub memory_usage: AtomicUsize,
    }

    impl Arena {
        pub fn new() -> Arena {
            Arena { memory_usage: AtomicUsize::new(0) }
        }

        pub fn allocate(&self, bytes: usize) -> *mut u8 {
            todo!()
        }

        pub fn memory_usage(&self) -> usize {
            self.memory_usage.load(std::sync::atomic::Ordering::Relaxed)
        }

        pub fn allocate_aligned(&self, bytes: usize) -> *mut u8 {
            todo!()
        }
    }
}
