#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Pair<T, E> {
    pub first: T,
    pub second: E,
}

impl<T, E> Pair<T, E> {
    pub fn new(first: T, second: E) -> Pair<T, E> {
        Pair { first, second }
    }
}
