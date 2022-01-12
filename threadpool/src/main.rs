use std::thread;
use std::time::Duration;
use threadpool::threadpool::ThreadPoolFactory;

fn main() {
    let mut pool = ThreadPoolFactory::new_cached(Duration::from_secs(10));

    for i in 0..10 {
        thread::sleep(Duration::from_millis(1));
        pool.execute(move || {
            println!("hoooo {}", i);
            thread::sleep(Duration::from_millis(4))
        });
    }

    println!("{}", pool.completed_task_count());
}
