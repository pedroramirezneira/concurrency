use std::time::Instant;
use std::{env, thread, time::Duration};
use tokio::time::sleep;

#[allow(dead_code)]
fn leibniz_pi_partial(start: usize, count: usize) -> f64 {
    (start..start + count)
        .map(|k| {
            let k = k as f64;
            (-1.0f64).powf(k) / (2.0 * k + 1.0)
        })
        .sum::<f64>() * 4.0
}

#[allow(dead_code)]
fn thread_io_simulation(tasks: usize) {
    let start = Instant::now();
    let mut handles = vec![];
    for _ in 0..tasks {
     // spawneamos un thread para cada task y simulamos una tarea de i/o
        handles.push(thread::spawn(|| {
            thread::sleep(Duration::from_millis(100));
        }));
    }
    for handle in handles { handle.join().unwrap(); }
    println!("i/o simulation done in {:?}", start.elapsed());
}

#[allow(dead_code)]
fn thread_pi_computation(tasks: usize, terms: usize) {
    let start = Instant::now();
    let terms_per_task = terms / tasks;
    let mut handles = vec![];
    for i in 0..tasks {
        let start = i * terms_per_task;
        let count = if i == tasks - 1 {
            terms - start
        } else {
            terms_per_task
        }; // spawneamos un thread para cada tarea
        handles.push(thread::spawn(move || {
            leibniz_pi_partial(start, count)
        }));
    }
    let result: f64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("pi calculation done in {:?}, value is {:.10}", start.elapsed(), result);
}

#[allow(dead_code)]
async fn async_io_simulation(tasks: usize) {
    let start = Instant::now();
    let mut handles = vec![];
    for _ in 0..tasks {
     // spawneamos una tarea asincrónica para cada task y simulamos una tarea de i/o
        handles.push(tokio::spawn(async {
            sleep(Duration::from_millis(100)).await;
        }));
    }
    for handle in handles { handle.await.unwrap(); }
    println!("i/o simulation done in {:?}", start.elapsed());
}

#[allow(dead_code)]
async fn async_pi_computation(tasks: usize, terms: usize) {
    let start = Instant::now();
    let terms_per_task = terms / tasks;
    let mut handles = vec![];
    for i in 0..tasks {
        // spawneamos una tarea asincrónica para cada task
        handles.push(tokio::spawn(async move {
            leibniz_pi_partial(i * terms_per_task, terms_per_task)
        }));
    }
    let mut result = 0.0;
    for handle in handles { result += handle.await.unwrap(); }
    println!("pi calculation done in {:?}, value is {:.10}", start.elapsed(), result);
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let tasks = args
        .iter()
        .position(|x| x == "--tasks")
        .map(|i| args[i + 1]
            .parse()
            .unwrap())
        .unwrap_or(2);
    let terms = args
        .iter()
        .position(|x| x == "--terms")
        .map(|i| args[i + 1]
            .parse()
            .unwrap())
        .unwrap_or(2);


    println!("Running with {} tasks, {} terms", tasks, terms);
    println!("-=-=- ASYNC TASKS -=-=-");
    async_io_simulation(tasks).await;
    // async_pi_computation(tasks, terms).await;
    println!("-=-=- THREADED TASKS -=-=-");
    thread_io_simulation(tasks);
    // thread_pi_computation(tasks, terms);
}
