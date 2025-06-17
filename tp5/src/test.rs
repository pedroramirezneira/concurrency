use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Instant;
use crate::queue::Queue;

pub fn run<Q: Queue<u32> + Sync + Send + 'static>(
    queue: Arc<Q>,
    producers: usize,
    consumers: usize,
    items_per_producer: usize,
) {
    let total_items = producers * items_per_producer;
    let mut handles = vec![];

    for id in 0..producers {
        let q = Arc::clone(&queue);
        handles.push(thread::spawn(move || {
            for i in 0..items_per_producer {
                q.enqueue(i as u32);
            }
            println!("Producer {} finished", id);
        }));
    }
    let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    for id in 0..consumers {
        let q = Arc::clone(&queue);
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            loop {
                if let Some(d) = q.dequeue() {
                    println!("Consumer #{} dequeued {}", id, d);
                    let prev = c.fetch_add(1, Ordering::Relaxed);
                    if prev + 1 >= total_items {
                        break;
                    }
                }
            }
            println!("Consumer {} finished", id);
        }));
    }

    let start = Instant::now();
    for handle in handles {
        handle.join().unwrap();
    }
    let elapsed = start.elapsed();
    println!("Test finished in {:?}", elapsed);
}
