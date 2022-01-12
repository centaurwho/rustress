pub mod threadpool;
mod worker;

pub enum Message {
    NewJob(Job),
    Terminate,
}

// TODO: Change return type of jobs to allow error handling, currently thread just panicks
pub type Job = Box<dyn FnOnce() + Send + 'static>;