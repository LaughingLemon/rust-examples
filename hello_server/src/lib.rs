use std::thread;

pub struct ThreadPool {
    size: u32
}

impl ThreadPool {
    pub fn execute<F>(&self, lambda: F)
        where F: FnOnce() + Send + 'static {
        thread::spawn(lambda);
    }
}

impl ThreadPool {
    pub fn new(pool_size: usize) -> ThreadPool {
        ThreadPool {
            size: pool_size as u32
        }
    }
}

