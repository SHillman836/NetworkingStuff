mod queue;

use queue::spawn_task;
use queue::sleeping;
use futures_lite::future;
use std::time::{Duration, Instant};


fn main() {
    let handle_one = spawn_task(sleeping(1));
    let handle_two = spawn_task(sleeping(2));
    let handle_three = spawn_task(sleeping(3));
    println!("before the sleep");
    std::thread::sleep(Duration::from_secs(5));
    println!("before the block");
    future::block_on(handle_one);
    future::block_on(handle_two);
    future::block_on(handle_three);
}