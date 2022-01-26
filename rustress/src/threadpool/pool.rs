use std::sync::{Arc, mpsc, Mutex};

use crate::threadpool::{Job, Message};
use crate::threadpool::pool::worker::Worker;

mod worker;

pub struct ThreadPool {
    max_pool_size: usize,

    workers: Vec<Worker>,

    sender: mpsc::Sender<Message>,
    receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
}

impl ThreadPool {
    pub fn create_worker(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        Worker::new(id, receiver)
    }

    pub fn execute<F>(&mut self, f: F)
        where F: FnOnce() + Send + 'static {
        let job = Message::NewJob(Box::new(f));

        // Possible concurrency problems here. If execute is called multiple times in a short
        // span of time, then self.no_available_worker() may not be changed quick enough. i.e After
        // the first execute call, a worker is created. However, it does some bookkeeping before
        // executing the task and switching to busy, so will seem available.
        if self.max_pool_size > self.workers.len() && self.no_available_worker() {
            let id = self.workers.len();
            let mut new_worker = ThreadPool::create_worker(id, Arc::clone(&self.receiver));
            new_worker.start();
            self.workers.push(new_worker);
        }
        self.sender.send(job).unwrap();
    }

    // Is just an estimate if there are working threads. Use with caution
    pub fn completed_task_count(&self) -> u32 {
        (&self.workers).iter()
            .map(|w| w.completed_task_count())
            .sum()
    }

    fn no_available_worker(&self) -> bool {
        self.workers.is_empty() || (&self.workers).iter()
            .all(|w| w.is_busy())
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

    pub fn new_lazy() -> ThreadPool {
        ThreadPoolBuilder::new()
            .max_pool_size(usize::MAX)
            .build()
    }
}

pub struct ThreadPoolBuilder {
    max_pool_size: usize,
    active_pool_size: usize,
    initial_job: Option<Job>,
}

impl ThreadPoolBuilder {
    pub fn new() -> ThreadPoolBuilder {
        ThreadPoolBuilder {
            max_pool_size: 1,
            active_pool_size: 0,
            initial_job: None,
        }
    }

    pub fn max_pool_size(mut self, n: usize) -> ThreadPoolBuilder {
        assert!(n > 0);
        assert!(n >= self.active_pool_size);
        self.max_pool_size = n;
        self
    }

    pub fn active_pool_size(mut self, n: usize) -> ThreadPoolBuilder {
        assert!(n > 0);
        assert!(n <= self.max_pool_size);
        self.active_pool_size = n;
        self
    }

    // TODO: Add tests for this, then remove this macro
    #[allow(unused)]
    pub fn initial_job(mut self, initial_job: Job) -> ThreadPoolBuilder {
        self.initial_job = Some(initial_job);
        self
    }

    pub fn build(self) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = (0..self.active_pool_size)
            .map(|id| ThreadPool::create_worker(id, Arc::clone(&receiver)))
            .collect();

        for worker in workers.iter_mut() {
            worker.start()
        }

        // TODO: Logic for initial job needed to run by all workers
        if let Some(job) = self.initial_job {
            sender.send(Message::NewJob(job)).unwrap();
        }

        ThreadPool {
            max_pool_size: self.max_pool_size,
            workers,
            sender,
            receiver: receiver.clone(),
        }
    }
}

// TODO: add tests
