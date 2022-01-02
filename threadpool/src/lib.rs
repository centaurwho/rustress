pub enum Message {
    NewJob(Job),
    Terminate,
}

pub type Job = Box<dyn FnOnce() + Send + 'static>;