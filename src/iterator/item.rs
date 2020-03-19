use crate::Node;

/// Item iterated over using an iterator.
pub struct Item<'a, T> {
    pub node: &'a Node<T>,
    pub level: usize,
}
