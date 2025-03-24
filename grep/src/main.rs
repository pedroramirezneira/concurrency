use grep ::search::{ChunkedConcurrentSearch, ConcurrentSearch, SequentialSearch};
use grep::SearchStrategy;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Uso: {} <modo> <patrón> <archivo1> [archivo2 ...]", args[0]);
        return;
    }

    let mode = &args[1];
    let pattern = &args[2];
    let files = &args[3..];

    let search_strategy: Box<dyn SearchStrategy> = match mode.as_str() {
        "seq" => Box::new(SequentialSearch),
        "conc" => Box::new(ConcurrentSearch),
        "c-chunk" => Box::new(ChunkedConcurrentSearch { chunk_size: 1024 }), // 1 KB por chunk
        _ => {
            eprintln!("Modo no válido. Usa 'seq', 'conc' o 'c-chunk'.");
            return;
        }
    };

    for file in files {
        search_strategy.search(file, pattern);
    }
}

