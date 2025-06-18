use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use crate::queue::blocking::BlockingQueue;
use crate::queue::Queue;

pub(crate) fn run_blocking<Q: Queue<usize> + Send + Sync + 'static>(
    queue: Arc<Q>,
    producers: usize,
    consumers: usize,
    items: usize
) {
    let total_items = Arc::new(AtomicUsize::new(items*producers));
    let start = std::time::Instant::now();

    println!("-=-=- BLOCKING QUEUE -=-=-");
    thread::scope(|s| {
        let queue = Arc::new(BlockingQueue::new());

        for producer_id in 0..producers {
            let prod_queue = Arc::clone(&queue);
            s.spawn(move || {
                for _i in 0..items {
                    prod_queue.enqueue(1);
                }

                println!("Producer {} finished enqueuing", producer_id);
            });
        }

        for id in 0..consumers {
            let cons_queue = Arc::clone(&queue);
            let remaining = Arc::clone(&total_items);

            s.spawn(move || {
                let thread_start = std::time::Instant::now();
                let mut count = 0;

                loop {
                    let current = remaining.load(Ordering::SeqCst);
                    if current == 0 {
                        break;
                    }
                    if remaining.compare_exchange(current, current - 1, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                        match cons_queue.dequeue() {
                            Some(_) => {
                                count += 1;
                            }
                            None => continue,
                        }
                    }
                }
                let duration = thread_start.elapsed();
                println!(
                    "Consumer {}: Dequeued {} items in {:?}",
                    id,
                    count,
                    duration
                );
            });

        }
    });

    let total_duration = start.elapsed();
    println!("Total time: {:?}", total_duration);
}

pub(crate) fn run_non_blocking<Q: Queue<usize> + Send + Sync + 'static>(
    queue: Arc<Q>,
    producers: usize,
    consumers: usize,
    items: usize
) {
    let total_items = producers * items;
    let consumed = Arc::new(AtomicUsize::new(0));
    let start = std::time::Instant::now();

    println!("-=-=- NON BLOCKING QUEUE -=-=-");
    let mut handles = Vec::new();

    for producer_id in 0..producers {
        let q = Arc::clone(&queue);
        handles.push(thread::spawn(move || {
            for j in 0..items {
                q.enqueue(producer_id * items + j);
            }

            println!("Producer {} finished", producer_id);
        }));
    }

    for i in 0..consumers {
        let q = Arc::clone(&queue);
        let consumed_clone = Arc::clone(&consumed);
        handles.push(thread::spawn(move || {
            let thread_start = std::time::Instant::now();
            let mut count = 0;

            while consumed_clone.load(Ordering::Acquire) < total_items {
                if let Some(_) = q.dequeue() {
                    consumed_clone.fetch_add(1, Ordering::AcqRel);
                    count += 1;
                } else {
                    thread::yield_now();
                }
            }

            let duration = thread_start.elapsed();
            println!(
                "Consumer {}: Dequeued {} items in {:?}",
                i,
                count,
                duration,
            );
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let total_duration = start.elapsed();
    println!("Total time: {:?}", total_duration);
}

