use std::sync::{Arc, mpsc, Mutex};
use std::time::Duration;

use crate::{Job, Message};
use crate::worker::Worker;

pub struct ThreadPool {
    pool_settings: PoolSettings,
    state: State,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

pub struct PoolSettings {
    // TODO: use this
    max_pool_count: usize,
    // TODO: Find a way for worker to notify thread_pool so we can update this after
    //  every completed job
    completed_task_count: u64,
    thread_fn: Option<ThreadProducer>,
    // TODO: use this
    keep_alive_time: Option<Duration>,
}

impl ThreadPool {
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        let job = Message::NewJob(Box::new(f));
        // TODO: increment worker count here if needed
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");
        self.state = advance_state(&self.state);

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

#[derive(Debug)]
pub enum State {
    // TODO: Decide all states and when to advance
    Running(usize),
    Cleaning,
    Terminated,
}

fn advance_state(s: &State) -> State{
    match s {
        State::Running(_) => State::Cleaning,
        State::Cleaning => State::Terminated,
        State::Terminated => unreachable!()
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

    pub fn new_cached(time: Duration) -> ThreadPool {
        ThreadPoolBuilder::new()
            .max_pool_size(usize::MAX)
            .keep_alive_time(time)
            .build()
    }
}

// TODO: Raw function pointer vs Boxed trait object vs type parameter
type ThreadProducer = fn() -> std::thread::Thread;

struct ThreadPoolBuilder {
    max_pool_size: usize,
    active_pool_size: usize,
    keep_alive_time: Option<Duration>,
    thread_fn: Option<ThreadProducer>,
    initial_job: Option<Job>,
}

impl ThreadPoolBuilder {
    fn new() -> ThreadPoolBuilder {
        ThreadPoolBuilder {
            max_pool_size: 1,
            active_pool_size: 0,
            keep_alive_time: None,
            thread_fn: None,
            initial_job: None,
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

    fn initial_job(mut self, initial_job: Job) -> ThreadPoolBuilder {
        self.initial_job = Some(initial_job);
        self
    }

    fn build(self) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..self.active_pool_size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect();

        let pool_settings = PoolSettings {
            max_pool_count: self.max_pool_size,
            completed_task_count: 0,
            thread_fn: self.thread_fn,
            keep_alive_time: self.keep_alive_time,
        };

        // TODO: Logic for initial job needed to run by all workers
        if let Some(job) = self.initial_job {
            sender.send(Message::NewJob(job));
        }

        ThreadPool {
            pool_settings,
            state: State::Running(self.active_pool_size),
            workers,
            sender,
        }
    }
}

// TODO: add tests