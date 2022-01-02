use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use crate::threadpool::ThreadPool;

mod worker;
mod threadpool;

fn main() {
    let pool = ThreadPool::new(4);

    for i in 0..10 {
        pool.execute(move || {
            println!("hoooo {}", i);
        })
    }

    thread::sleep(Duration::from_secs(2));
}
