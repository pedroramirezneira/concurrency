use std::{
    collections::{HashSet, VecDeque},
    sync::{Arc, atomic::{AtomicBool, Ordering}},
};
use std::sync::atomic::AtomicUsize;
use tokio::sync::{mpsc, Mutex};

type Url = String;

struct SharedState {
    visited: Mutex<HashSet<Url>>,
    parents: Mutex<std::collections::HashMap<Url, Url>>,
    total_links: AtomicUsize,
    found: AtomicBool,
}

async fn run_bfs(start: Url, target: Url, actor_count: usize) {
    let (tx, mut rx) = mpsc::channel::<Url>(10000);
    let state = Arc::new(SharedState {
        visited: Mutex::new(HashSet::new()),
        parents: Mutex::new(std::collections::HashMap::new()),
        total_links: AtomicUsize::new(0),
        found: AtomicBool::new(false),
    });

    tx.send(start.clone()).await.unwrap();

    for _ in 0..actor_count {
        let state = Arc::clone(&state);
        let mut rx = rx.clone();
        let tx = tx.clone();
        let target = target.clone();

        tokio::spawn(async move {
            while let Some(current) = rx.recv().await {
                if state.found.load(Ordering::SeqCst) {
                    break;
                }

                let mut visited = state.visited.lock().await;
                if !visited.insert(current.clone()) {
                    continue; // already visited
                }

                drop(visited); // release lock

                let links = extract_links(&current).await;

                for link in links {
                    state.total_links.fetch_add(1, Ordering::SeqCst);

                    if link == target {
                        state.found.store(true, Ordering::SeqCst);
                        let mut parents = state.parents.lock().await;
                        parents.insert(link.clone(), current.clone());
                        break;
                    }

                    let mut parents = state.parents.lock().await;
                    if !parents.contains_key(&link) {
                        parents.insert(link.clone(), current.clone());
                        tx.send(link).await.unwrap();
                    }
                }
            }
        });
    }

    // esperar hasta que se encuentre el objetivo
    loop {
        if state.found.load(Ordering::SeqCst) {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    // reconstruir camino
    let path = reconstruct_path(&state, start, target).await;
    for (i, step) in path.iter().enumerate() {
        println!("{}. {}", i + 1, step);
    }
    println!("Total links processed: {}", state.total_links.load(Ordering::SeqCst));
}
