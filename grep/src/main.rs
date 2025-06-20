use grep::SearchStrategy;
use grep::search::{ChunkedConcurrentSearch, ConcurrentSearch, SequentialSearch};
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

    let file_list: Vec<String> = files.iter().map(|s| s.to_string()).collect();
    search_strategy.search(&file_list, pattern);
}
