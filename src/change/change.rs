use crate::Node;

/// Change events which can happen in the tree.
pub enum Event<'a, T> {
    NodeAdded {
        parent: &'a Node<T>,
        node: &'a Node<T>,
    },
    NodeRemoved {
        parent: &'a Node<T>,
        removed_idx: usize,
    },
    TextChanged {
        node: &'a Node<T>,
    },
}
