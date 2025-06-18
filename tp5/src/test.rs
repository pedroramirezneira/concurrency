use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use crate::queue::Queue;

pub(crate) fn run<Q: Queue<usize> + Send + Sync + 'static>(
    queue: Arc<Q>,
    producers: usize,
    consumers: usize,
    items_per_producer: usize,
    blocking: bool
) {
    let total_items = producers * items_per_producer;
    let remaining = Arc::new(AtomicUsize::new(total_items));
    let start = std::time::Instant::now();

    thread::scope(|s| {
        // Spawn producers
        for id in 0..producers {
            let q = Arc::clone(&queue);
            s.spawn(move || {
                let thread_start = std::time::Instant::now();
                for _ in 0..items_per_producer {
                    q.enqueue(1);
                }
                let duration = thread_start.elapsed();
                println!("Producer {}: done in {:?} ({:.2} items/sec)", id, duration, items_per_producer as f64 / duration.as_secs_f64());
            });
        }


        // Spawn consumers
        for id in 0..consumers {
            let q = Arc::clone(&queue);
            let remaining = Arc::clone(&remaining);

            s.spawn(move || {
                let thread_start = std::time::Instant::now();
                let mut count = 0;
                let mut sum = 0;

                while remaining.load(Ordering::Acquire) > 0 {
                    let maybe_item = q.dequeue();
                    match maybe_item {
                        Some(val) => {
                            count += 1;
                            sum += val;
                            remaining.fetch_sub(1, Ordering::AcqRel);
                        }
                        None => {
                            if !blocking {
                                thread::yield_now(); // For non-blocking queues, yield
                            }
                        }
                    }
                }

                let duration = thread_start.elapsed();
                println!("Consumer {}: Dequeued {} items in {:?} ({:.2} items/sec), Sum: {}",
                         id, count, duration, count as f64 / duration.as_secs_f64(), sum);
            });
        }
    });

    let duration = start.elapsed();
    println!("Total time: {:?}", duration);
}
