use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr::null_mut;
use crate::queue::Queue;

struct Node<T> {
    item: Option<T>,
    next: AtomicPtr<Node<T>>,
}

pub struct NonBlockingQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}

impl<T> NonBlockingQueue<T> {
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(Node {
            item: None,
            next: AtomicPtr::new(null_mut()),
        }));

        NonBlockingQueue {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
        }
    }
}

impl<T: Send> Queue<T> for NonBlockingQueue<T> {

    fn enqueue(&self, item: T) {
        let new_node = Box::into_raw(Box::new(Node {
            item: Some(item),
            next: AtomicPtr::new(null_mut()),
        }));

        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };

            if tail == self.tail.load(Ordering::Acquire) {
                if next.is_null() {
                    if unsafe { (*tail).next.compare_exchange(null_mut(), new_node, Ordering::AcqRel, Ordering::Relaxed).is_ok() } {
                        let _ = self.tail.compare_exchange(tail, new_node, Ordering::AcqRel, Ordering::Relaxed);
                        return;
                    }
                } else {
                    let _ = self.tail.compare_exchange(tail, next, Ordering::AcqRel, Ordering::Relaxed);
                }
            }
        }
    }

    fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*head).next.load(Ordering::Acquire) };

            if head == self.head.load(Ordering::Acquire) {
                if next.is_null() {
                    if head == tail {
                        return None; // Empty
                    }
                    // Help advance the tail
                    let _ = self.tail.compare_exchange(tail, next, Ordering::AcqRel, Ordering::Relaxed);
                } else {
                    if self.head.compare_exchange(head, next, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
                        let result = unsafe { (*next).item.take() };
                        let _ = unsafe { Box::from_raw(head) };
                        return result;
                    }
                }
            }
        }
    }
}