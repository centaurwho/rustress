use std::sync::{Arc, mpsc, Mutex};
use std::thread;

use crate::{Job, Message};

pub struct Worker {
    pub id: usize,
    receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
    initial_job: Arc<Mutex<Option<Job>>>,
    // TODO: Implement
    completed_jobs: u32,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>, initial_job: Arc<Mutex<Option<Job>>>) -> Worker {
        Worker {
            id,
            receiver,
            initial_job,
            completed_jobs: 0,
            thread: None,
        }
    }

    // TODO: Check if I can get rid of this static lifetime
    pub fn start(&'static mut self) {
        let thread = thread::spawn(|| run(self.id, &self.receiver));
        self.thread = Some(thread);
    }
}

fn run(id: usize, receiver: &Arc<Mutex<mpsc::Receiver<Message>>>) {
    loop {
        let message = receiver.lock().unwrap().recv().unwrap();
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
    }
}