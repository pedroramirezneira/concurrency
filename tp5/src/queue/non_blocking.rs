use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr::null_mut;
use crate::queue::Queue;

struct Node<T> {
    item: Option<T>,
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(item: Option<T>) -> *mut Node<T> {
        let boxed = Box::new(Node {
            item,
            next: AtomicPtr::new(null_mut()),
        });
        Box::into_raw(boxed)
    }
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
        let new_node = Node::new(Some(item));

        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };

         // si el next de tail es nulo significa que estamos en la tail
         // verdadera, entonces podemos intentar enlazar el nuevo nodo
            if next.is_null() {
             // Intentamos hacer que el next de la tail sea el
             // nuevo nodo, solo si este next sigue siendo nulo
                if unsafe { (*tail).next.compare_exchange(
                    null_mut(),
                    new_node,
                    Ordering::AcqRel,
                    Ordering::Relaxed,
                ).is_ok() } {
                 // intentamos mover el puntero de tail al nuevo nodo
                 // manteniendo la tail actualizada, ayudando a otros hilos.
                    self.tail.compare_exchange(
                        tail,
                        new_node,
                        Ordering::Release,
                        Ordering::Relaxed,
                    ).ok();
                    return;
                }
         // si tail.next no era nulo, es porque otro thread ya agregó un nodo
            } else {
             // entonces lo intentamos ayudar actualizando tail
                self.tail.compare_exchange(
                    tail,
                    next,
                    Ordering::Release,
                    Ordering::Relaxed,
                ).ok();
            }
        }
    }

    fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*head).next.load(Ordering::Acquire) };

         // si next es nulo, la queue está
         // vacía, ya que no hay nodos reales
            if next.is_null() {
                return None;
            }

         // si head y tail son iguales, significa que la tail está desactualizada
         // todavia apuntando al dummy, ayudamos actualizando la tail
            if head == tail {
                self.tail.compare_exchange(
                    tail,
                    next,
                    Ordering::Release,
                    Ordering::Relaxed,
                ).ok();
         // si head != tail, hay al menos un nodo real, intentamos actualizar head
            } else {
                if self.head.compare_exchange(
                    head,
                    next,
                    Ordering::Release,
                    Ordering::Relaxed,
                ).is_ok() {
                 // si el CAS es exitoso, agarramos el item del nodo y lo devolvemos
                    return unsafe { (*next).item.take() };
                }
            }
        }
    }
}