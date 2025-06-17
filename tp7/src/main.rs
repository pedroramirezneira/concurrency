use std::time::Instant;
use std::{thread, time::Duration};

use tokio::time::sleep;

fn leibniz_pi_partial(start: usize, count: usize) -> f64 {
    (start..start + count)
        .map(|k| {
            let k = k as f64;
            (-1.0f64).powf(k) / (2.0 * k + 1.0)
        })
        .sum::<f64>() * 4.0
}

fn thread_io_simulation(tasks: usize) {
    let start = Instant::now();

    let mut handles = vec![];
    for _ in 0..tasks {
        handles.push(thread::spawn(|| {
            thread::sleep(Duration::from_millis(100));
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("i/o simulation done in {:?}", start.elapsed());
}

fn thread_pi_computation(tasks: usize, terms: usize) {
    let start = Instant::now();
    let terms_per_task = terms / tasks;

    let mut handles = vec![];
    for i in 0..tasks {
        handles.push(thread::spawn(move || {
            leibniz_pi_partial(i * terms_per_task, terms_per_task)
        }));
    }

    let result: f64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("pi calculation done in {:?}, value is {:.10}", start.elapsed(), result);
}

// ASYNC VERSION
async fn async_io_simulation(tasks: usize) {
    let start = Instant::now();

    let mut handles = vec![];
    for _ in 0..tasks {
        handles.push(tokio::spawn(async {
            sleep(Duration::from_millis(100)).await;
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("i/o simulation done in {:?}", start.elapsed());
}

async fn async_pi_computation(tasks: usize, terms: usize) {
    let start = Instant::now();
    let terms_per_task = terms / tasks;

    let mut handles = vec![];
    for i in 0..tasks {
        handles.push(tokio::spawn(async move {
            leibniz_pi_partial(i * terms_per_task, terms_per_task)
        }));
    }

    let mut result = 0.0;
    for handle in handles {
        result += handle.await.unwrap();
    }

    println!("pi calculation done in {:?}, value is {:.10}", start.elapsed(), result);
}

#[tokio::main]
async fn main() {
    let tasks = 100_000;
    let terms = 1_000_000;

    println!("-=-=- THREADED TASKS -=-=-");
    thread_io_simulation(tasks);
    thread_pi_computation(tasks, terms);
    println!();
    println!("-=-=- ASYNC TASKS -=-=-");
    async_io_simulation(tasks).await;
    async_pi_computation(tasks, terms).await;
}
