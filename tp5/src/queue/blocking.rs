use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

/// Cola concurrente bloqueante.
pub struct BlockingQueue<T> {
    queue: Mutex<VecDeque<T>>,
    condvar: Condvar,
}

impl<T> BlockingQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
        }
    }
}

impl<T: Send> super::Queue<T> for BlockingQueue<T> {
    fn enqueue(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(item);
        // Notifica a un posible consumidor bloqueado
        self.condvar.notify_one();
    }

    fn dequeue(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        // Espera mientras la cola esté vacía
        while queue.is_empty() {
            queue = self.condvar.wait(queue).unwrap();
        }
        queue.pop_front()
    }
}
