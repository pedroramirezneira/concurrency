use std::fs::File;
use std::io::{self, BufRead, Seek, SeekFrom};
use std::sync::Arc;
use std::thread;

use crate::SearchStrategy;
use crate::bruteforce::bruteforce;

pub struct SequentialSearch;

impl SearchStrategy for SequentialSearch {
    fn search(&self, file_path: &str, pattern: &str) {
        let mut line_number = 0;
        if let Ok(file) = File::open(file_path) {
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                line_number += 1;
                if let Ok(line) = line {
                    if bruteforce(&line, pattern) {
                        println!("{}", line_number);
                    }
                }
            }
        }
    }
}

pub struct ConcurrentSearch;

impl SearchStrategy for ConcurrentSearch {
    fn search(&self, file_path: &str, pattern: &str) {
        let mut line_number = 0;
        let pattern = Arc::new(pattern.to_string());

        thread::spawn({
            let file_path = file_path.to_string();
            let pattern = Arc::clone(&pattern);

            move || {
                if let Ok(file) = File::open(&file_path) {
                    let reader = io::BufReader::new(file);
                    for line in reader.lines() {
                        line_number += 1;
                        if let Ok(line) = line {
                            if bruteforce(&line, &pattern) {
                                println!("{}", line_number);
                            }
                        }
                    }
                }
            }
        })
        .join()
        .unwrap();
    }
}

pub struct ChunkedConcurrentSearch {
    pub chunk_size: usize, // Tamaño de cada chunk en bytes
}

impl SearchStrategy for ChunkedConcurrentSearch {
    fn search(&self, file_path: &str, pattern: &str) {
        let file = File::open(file_path).expect("Error abriendo el archivo");
        let file_size = file.metadata().unwrap().len() as usize;

        let pattern = Arc::new(pattern.to_string());

        let mut handles = vec![];
        let mut start = 0;
        let mut line_number = 0;
        while start < file_size {
            let chunk_size = self.chunk_size.min(file_size - start);
            let pattern = Arc::clone(&pattern);
            let file_path = file_path.to_string();

            let handle = thread::spawn(move || {
                let mut file = File::open(&file_path).unwrap();
                file.seek(SeekFrom::Start(start as u64)).unwrap();
                let reader = io::BufReader::new(file);

                for line in reader.lines().take(chunk_size) {
                    line_number += 1;
                    if let Ok(line) = line {
                        if bruteforce(&line, &pattern) {
                            println!("{}", line_number);
                        }
                    }
                }
            });

            handles.push(handle);
            start += chunk_size;
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
