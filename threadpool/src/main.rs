use std::thread;
use std::time::Duration;

use crate::threadpool::ThreadPoolFactory;

mod worker;
mod threadpool;

fn main() {
    let pool = ThreadPoolFactory::new_fixed_sized(4);

    for i in 0..10 {
        pool.execute(move || {
            println!("hoooo {}", i);
        })
    }

    thread::sleep(Duration::from_secs(2));
}
