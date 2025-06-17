use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;

/// Nodo de la cola
struct Node<T> {
    value: Option<T>,
    next: AtomicPtr<Node<T>>,
}

/// Cola concurrente no bloqueante
pub struct NonBlockingQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}

impl<T> NonBlockingQueue<T> {
    pub fn new() -> Self {
        // Nodo dummy inicial
        let dummy = Box::into_raw(Box::new(Node {
            value: None,
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        Self {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
        }
    }
}

impl<T: Send> super::Queue<T> for NonBlockingQueue<T> {
    fn enqueue(&self, item: T) {
        let new_node = Box::into_raw(Box::new(Node {
            value: Some(item),
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };

            if next.is_null() {
                // Intentamos enlazar el nuevo nodo
                if unsafe { (*tail).next.compare_exchange(
                    ptr::null_mut(),
                    new_node,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                ).is_ok() } {
                    // Intentamos avanzar el tail
                    let _ = self.tail.compare_exchange(
                        tail,
                        new_node,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    );
                    break;
                }
            } else {
                // Otro hilo ya insertó un nodo, avanzamos tail
                let _ = self.tail.compare_exchange(
                    tail,
                    next,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                );
            }
        }
    }

    fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*head).next.load(Ordering::Acquire) };

            if next.is_null() {
                return None; // Cola vacía
            }

            if head == tail {
                // Otro hilo está en medio de un enqueue; intentamos avanzar tail
                let _ = self.tail.compare_exchange(
                    tail,
                    next,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                );
            } else {
                // Obtenemos el valor y avanzamos head
                let value = unsafe { (*next).value.take() };
                if self.head.compare_exchange(
                    head,
                    next,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                ).is_ok() {
                    // Liberamos nodo anterior
                    unsafe { drop(Box::from_raw(head)) };
                    return value;
                }
            }
        }
    }
}

impl<T> Drop for NonBlockingQueue<T> {
    fn drop(&mut self) {
        // Liberar todos los nodos para evitar fugas de memoria
        let mut current = self.head.load(Ordering::Relaxed);
        while !current.is_null() {
            let next = unsafe { (*current).next.load(Ordering::Relaxed) };
            unsafe { drop(Box::from_raw(current)) };
            current = next;
        }
    }
}

