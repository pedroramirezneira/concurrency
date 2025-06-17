pub mod blocking;
pub mod non_blocking;

pub trait Queue<T: Send> {
    fn enqueue(&self, item: T);
    fn dequeue(&self) -> Option<T>;
}
