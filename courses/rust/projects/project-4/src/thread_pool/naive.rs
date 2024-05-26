use std::thread;
use crate::thread_pool::ThreadPool;

/// It is actually not a thread pool. It spawns a new thread every time
/// the `spawn` method is called.
pub struct NaiveThreadPool;

impl ThreadPool for NaiveThreadPool {
    fn new(threads: u32) -> crate::Result<Self> where Self: Sized {
        Ok(NaiveThreadPool)
    }

    fn spawn<F>(&self, job: F) where F: FnOnce() + Send + 'static {
        thread::spawn(job);
    }
}