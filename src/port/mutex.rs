pub struct MutexGuard<'a, T> {
    guard: std::sync::MutexGuard<'a, T>,
}

impl<'a, T> MutexGuard<'a, T> {
    pub fn new(guard: std::sync::MutexGuard<'a, T>) -> Self {
        Self { guard }
    }
}

#[derive(Debug)]
pub struct Mutex<T> {
    mutex: std::sync::Mutex<T>,
}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            mutex: std::sync::Mutex::new(t),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        MutexGuard::new(self.mutex.lock().unwrap())
    }
}

pub struct CondVar {
    condvar: std::sync::Condvar,
}

impl CondVar {
    pub fn new() -> Self {
        Self {
            condvar: std::sync::Condvar::new(),
        }
    }

    pub fn wait(&self, guard: &mut MutexGuard<bool>) {}

    pub fn notify_one(&self) {}
}
