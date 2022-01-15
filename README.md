# rustress

Simple network stress testing library. To get familiar with Rust 

### Initial Road Map
- [ ] Customizable thread pool
- [ ] Concurrent client/server
- [ ] More in-depth config options
  - [ ] Adjusting throughput
  - [ ] Network layer protocol support (TCP, UDP etc..)
  - [ ] App layer protocol support (HTTP, FTP etc..)
  - [ ] Ability to define custom protocols or message formats
- [ ] Fetching or loading data to send
- [ ] Custom data generation
- [ ] Integrity testing
- [ ] Performance testing
- [ ] Custom network configurations instead of a simple server-client


### Features (So Far)

#### Thread Pool

```rust
use rustress::threadpool::pool::ThreadPoolFactory;

fn main() {
    // Create a cached pool (that will increase thread counts as needed.)
    let mut pool = ThreadPoolFactory::new_cached(Duration::from_secs(10));
    
    for i in 0..10 {
        // Submit the task to the pool. It will either create a thread or use an existing idle one 
        pool.execute(move || {
            println!("hoooo {}", i);
            thread::sleep(Duration::from_seconds(1))
        });
    }

    // This will print 10
    println!("{}", pool.completed_task_count());
    
    // Threads will be dropped here at the end of the scope
}
```