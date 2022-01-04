use std::sync::{Arc, mpsc, Mutex};
use std::time::Duration;

use crate::Message;
use crate::worker::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
    // TODO: capacity, completed_task_count, keep_alive_time
}

impl ThreadPool {
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        let job = Message::NewJob(Box::new(f));
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub struct ThreadPoolFactory;

impl ThreadPoolFactory {
    pub fn new_single_threaded() -> ThreadPool {
        ThreadPoolFactory::new_fixed_sized(1)
    }

    pub fn new_fixed_sized(thread_count: usize) -> ThreadPool {
        ThreadPoolBuilder::new()
            .max_pool_size(thread_count)
            .active_pool_size(thread_count)
            .build()
    }

    pub fn new_cached() -> ThreadPool {
        todo!()
    }

    pub fn new_scheduled() {
        todo!()
    }
}

// TODO: Raw function pointer vs Boxed trait object vs type parameter
type ThreadProducer = fn() -> std::thread::Thread;

struct ThreadPoolBuilder {
    max_pool_size: usize,
    active_pool_size: usize,
    keep_alive_time: Option<Duration>,
    thread_fn: Option<ThreadProducer>,
}

impl ThreadPoolBuilder {
    fn new() -> ThreadPoolBuilder {
        ThreadPoolBuilder {
            max_pool_size: 1,
            active_pool_size: 0,
            keep_alive_time: None,
            thread_fn: None,
        }
    }

    fn max_pool_size(mut self, n: usize) -> ThreadPoolBuilder {
        assert!(n > 0);
        assert!(n >= self.active_pool_size);
        self.max_pool_size = n;
        self
    }

    fn active_pool_size(mut self, n: usize) -> ThreadPoolBuilder {
        assert!(n > 0);
        assert!(n <= self.max_pool_size);
        self.active_pool_size = n;
        self
    }

    fn keep_alive_time(mut self, time: Duration) -> ThreadPoolBuilder {
        self.keep_alive_time = Some(time);
        self
    }

    fn thread_fn(mut self, thread_fn: ThreadProducer) -> ThreadPoolBuilder {
        self.thread_fn = Some(thread_fn);
        self
    }

    fn build(self) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..self.active_pool_size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect();

        ThreadPool {
            workers,
            sender,
        }
    }
}