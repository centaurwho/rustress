use std::sync::{Arc, mpsc, Mutex};
use std::time::Duration;

use crate::{Job, Message};
use crate::worker::Worker;

pub trait Executor<F> {
    fn execute(&self, f: F);
}

pub struct ThreadPool {
    pool_settings: PoolSettings,
    state: State,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

pub struct PoolSettings {
    max_pool_count: usize,
    completed_task_count: u64,
    thread_fn: Option<ThreadProducer>,
    keep_alive_time: Option<Duration>,
}


impl<F> Executor<F> for ThreadPool
    where F: FnOnce() + Send + 'static {

    fn execute(&self, f: F) {
        let job = Message::NewJob(Box::new(f));
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

fn advance_state(s: &State) -> State {
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
            initial_job: None
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
        // TODO: Consider logic for multiple channels to reduce congestion
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        // TODO: Sharing the same closure across multiple threads isn't a good idea probably. As long
        //  as the closure doesn't capture from outer scope this shouldn't be a big deal here
        //  Simplest solution would be just sending through the channel, but then only one worker can run the job
        //  Consider implementing a custom Job object implementing Clone
        //  Or make initial_job specific to workers
        let initial_job = Arc::new(Mutex::new(self.initial_job));

        let mut workers: Vec<Worker> = (0..self.active_pool_size)
            .map(|id| Worker::new(id, Arc::clone(&receiver), Arc::clone(&initial_job)))
            .collect();

        for worker in &workers {
            // println!("{}", worker.id);
            // worker.start()
        }

        let pool_settings = PoolSettings {
            max_pool_count: self.max_pool_size,
            completed_task_count: 0,
            thread_fn: self.thread_fn,
            keep_alive_time: self.keep_alive_time,
        };

        ThreadPool {
            pool_settings,
            state: State::Running(self.active_pool_size),
            workers,
            sender,
        }
    }
}