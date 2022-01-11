use std::sync::{Arc, mpsc, Mutex};
use std::thread;

use crate::{Job, Message};

pub struct Worker {
    pub id: usize,
    // TODO: Implement
    completed_jobs: u32,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // TODO: Try to extract thread spawning into a dedicated member function to separate
        //  worker creation and execution
        let thread = thread::spawn(move || loop {
            // TODO: error handling here. Dont panic
            let message = receiver.lock().unwrap().recv().unwrap();
            // TODO: If a worker is stuck doing a job, then relevant Message::Terminate will never
            //  be checked. Add some kind of timeout logic
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing", id);
                    job()
                }
                Message::Terminate => {
                    println!("Worker {} got terminate message.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            completed_jobs: 0,
            thread: Some(thread),
        }
    }
}