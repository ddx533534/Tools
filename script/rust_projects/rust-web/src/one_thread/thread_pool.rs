use std::thread;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl Worker {
    fn new(id: usize) -> Self {
        Self {
            id,
            thread: thread::spawn(|| {}),
        }
    }
}

pub struct ThreadPool {
    core_size: usize,
    threads: Vec<Worker>,
}
impl ThreadPool {
    pub fn new(core_size: usize) -> ThreadPool {
        let mut threads = Vec::with_capacity(core_size);
        for i in 0..core_size {
            threads.push(Worker::new(i));
        }
        Self {
            core_size,
            threads,
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}