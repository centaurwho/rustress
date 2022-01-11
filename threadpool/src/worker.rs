use std::sync::{Arc, mpsc, Mutex};
use std::thread;

use crate::{Message};

pub struct Worker {
    pub id: usize,
    receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
    completed_jobs: Arc<Mutex<u32>>,
    pub(crate) thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        Worker {
            id,
            receiver,
            completed_jobs: Arc::new(Mutex::new(0)),
            thread: None,
        }
    }

    pub fn completed_task_count(&self) -> u32 {
        *self.completed_jobs.lock().unwrap()
    }

    pub fn start(&mut self) {
        let id = self.id;
        let rec = self.receiver.clone();
        let job_count = self.completed_jobs.clone();

        let thread = thread::spawn(move || {
            println!("Hi, Worker {} started. ", id);
            loop {
                // TODO: error handling here. Dont panic
                let message = rec.lock().unwrap().recv().unwrap();
                // TODO: If a worker is stuck doing a job, then relevant Message::Terminate will never
                //  be checked. Add some kind of timeout logic
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing", id);
                        job();
                        *job_count.lock().unwrap() += 1;
                    },
                    Message::Terminate => {
                        println!("Worker {} got terminate message. Total completed task count: {}", id, job_count.lock().unwrap());
                        break;
                    }
                }
            }
        });

        self.thread = Some(thread);
    }
}