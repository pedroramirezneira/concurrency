pub mod bruteforce;
pub mod search;

pub trait SearchStrategy {
    fn search(&self, file_path: &str, pattern: &str);
}
