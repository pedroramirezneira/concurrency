use std::fs::File;
use std::io::{self, BufRead};
use std::sync::Arc;
use std::thread;

use crate::SearchStrategy;
use crate::bruteforce::bruteforce;

pub struct SequentialSearch;

impl SearchStrategy for SequentialSearch {
    fn search(&self, file_paths: &[String], pattern: &str) -> usize {
        let mut count = 0;
        for file_path in file_paths {
            let file = File::open(file_path).expect("Error opening file");
            let reader = io::BufReader::new(file);
            for (line_number, line) in reader.lines().enumerate() {
                if let Ok(line) = line {
                    if bruteforce(&line, pattern) {
                        println!("{}:{}", file_path, line_number + 1);
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

pub struct ConcurrentSearch;

impl SearchStrategy for ConcurrentSearch {

    fn search(&self, file_paths: &[String], pattern: &str) -> usize {
        let pattern = Arc::new(pattern.to_string());
        let mut handles = vec![];

        for file_path in file_paths {
            let file_path = file_path.clone();
            let pattern = Arc::clone(&pattern);

            let handle = thread::spawn(move || {
                let mut local_count = 0;
                if let Ok(file) = File::open(&file_path) {
                    let reader = io::BufReader::new(file);
                    for (line_number, line) in reader.lines().enumerate() {
                        if let Ok(line) = line {
                            if bruteforce(&line, &pattern) {
                                local_count += 1;
                                println!("{}:{}", file_path, line_number + 1);
                            }
                        }
                    }
                }
                local_count
            });

            handles.push(handle);
        }

        // Sum the results from each thread
        handles
            .into_iter()
            .map(|handle| handle.join().unwrap_or(0))
            .sum()
    }

}

pub struct ChunkedConcurrentSearch {
    pub chunk_size: usize,
}

impl SearchStrategy for ChunkedConcurrentSearch {
    fn search(&self, file_paths: &[String], pattern: &str) -> usize {
        let pattern = Arc::new(pattern.to_string());
        let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        for file_path in file_paths {
            let file_path = file_path.clone();
            let pattern = Arc::clone(&pattern);
            let file = File::open(&file_path).expect("Error opening the file");
            let reader = io::BufReader::new(file);
            let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
            let mut handles = vec![];
            for (chunk_index, chunk) in lines.chunks(self.chunk_size).enumerate() {
                let chunk = chunk.to_owned();
                let pattern = Arc::clone(&pattern);
                let count = Arc::clone(&count);
                let file_path = file_path.clone();
                let handle = thread::spawn(move || {
                    for (i, line) in chunk.iter().enumerate() {
                        if bruteforce(line, &pattern) {
                            let global_line_number = chunk_index * chunk.len() + i + 1;
                            println!("{}:{}", file_path, global_line_number);
                            count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        }
                    }
                });
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        }
        count.load(std::sync::atomic::Ordering::Relaxed)
    }
}
