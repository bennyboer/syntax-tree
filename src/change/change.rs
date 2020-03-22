use crate::Node;

/// Change events which can happen in the tree.
pub enum Event<'a, T> {
    NodeAdded {
        parent: &'a Node<T>,
        added_idx: usize,
    },
    NodeRemoved {
        parent: &'a Node<T>,
        removed_idx: usize,
    },
    InfosChanged {
        node: &'a Node<T>,
    },
    TextChanged {
        node: &'a Node<T>,
    },
}
