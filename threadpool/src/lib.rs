pub mod threadpool;
mod worker;

pub enum Message {
    // TODO: Other Message types: such as Message::Status, Message::ChangeConfig
    NewJob(Job),
    Terminate,
}

// TODO: Change return type of jobs to allow error handling, currently thread just panicks
pub type Job = Box<dyn FnOnce() + Send + 'static>;