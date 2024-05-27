use std::thread;

use crossbeam::{channel, Receiver, RecvError, Sender};
use log::{debug, error};

use crate::thread_pool::ThreadPool;

/// Note for rust training course: the thread pool is not implemented using
/// `catch_unwind` because it would require the task to be `UnwindSafe`
///
/// A thread pool using a shared queue inside
///
/// If a spawned task panics, the old thread will be destroyed and a new one will be
/// created. It fails silently when any failure to create the thread at the OS level
/// is captured after the thread pool is created. So, the thread number in the pool
/// can decrease to zero, then spawning a task to the thread pool will panic.
pub struct SharedQueueThreadPool {
    // a sender, it can send the values of type dyn FnOnce() + Send + 'static

    // Send ensure the closure can be sent between threads safely
    // 'static indicates the closure lifetime extend entire program's lifetime

    // tx is a sender used to send the closure to the pool
    sender: Sender<Box<dyn FnOnce() + Send + 'static>>,
}

impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> crate::Result<Self>
        where Self: Sized {
        let (sender, receiver) =

            channel// multiple producer and multiple consumer
            ::unbounded::<Box<dyn FnOnce() + Send + 'static>>();// creates a channel of unbounded capacity

        for _ in 0..threads {
            let rx = TaskReceiver(receiver.clone());
            thread::Builder::new().spawn(move || run_tasks(rx))?;
        }
        Ok(SharedQueueThreadPool { sender })
    }

    /// Spawns a function into the thread pool
    ///
    /// # Panics
    ///
    /// Panics if the thread pool has no thread.
    fn spawn<F>(&self, job: F)
        where F: FnOnce() + Send + 'static {
        self.sender
            .send(Box::new(job))
            .expect("The thread pool has no thread.");
    }
}

#[derive(Clone)]
struct TaskReceiver(Receiver<Box<dyn FnOnce() + Send + 'static>>);

impl Drop for TaskReceiver {
    fn drop(&mut self) {
        if thread::panicking() {
            let task_receiver = self.clone();
            if let Err(e) = thread::Builder::new().spawn(move || run_tasks(task_receiver)) {
                error!("Failed to spawn a thread: {}", e);
            }
        }
        todo!()
    }
}

fn run_tasks(task_receiver: TaskReceiver) {
    loop {
        match task_receiver.0.recv() {
            Ok(task) => {
                task();
            }
            Err(_) => {
                debug!("Thread exists because the thread pool is destroyed.");
            }
        }
    }
}