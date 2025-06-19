mod queue;
mod test;

use std::env;
use std::sync::Arc;
use crate::queue::non_blocking::NonBlockingQueue;

fn main() {
    let args: Vec<String> = env::args().collect();
    let producers = args.iter().position(|x| x == "--producers").map(|i| args[i + 1].parse().unwrap()).unwrap_or(2);
    let consumers = args.iter().position(|x| x == "--consumers").map(|i| args[i + 1].parse().unwrap()).unwrap_or(2);
    let items = args.iter().position(|x| x == "--items").map(|i| args[i + 1].parse().unwrap()).unwrap_or(1000);

    println!("Running with {} producers, {} consumers, {} items each", producers, consumers, items);

    let blocking_queue = Arc::new(queue::blocking::BlockingQueue::new());
    let non_blocking_queue = Arc::new(NonBlockingQueue::new());

    test::run_blocking(blocking_queue, producers, consumers, items);
    test::run_non_blocking(non_blocking_queue, producers, consumers, items);
}
