use std::fmt;
use crate::{Node, iterator, change};
use std::rc::Rc;
use std::hash::Hash;
use std::fmt::Debug;
use crate::change::Listener;

pub struct Tree<T> {
    /// The trees root node.
    root: Node<T>,

    /// List of listeners listening for change events.
    listeners: Option<Vec<change::Listener<T>>>,
}

impl<T> Tree<T>
    where T: Eq + Hash {
    /// Create new tree.
    pub fn new(string: &str) -> Tree<T> {
        Tree {
            root: Node::new_root(string),
            listeners: None,
        }
    }

    /// Load a tree.
    pub fn load() -> Tree<T> {
        // TODO Implement loading a tree from syntax/format information and text
        unimplemented!()
    }

    /// Set syntax/format info for the passed range.
    /// The range is the passed start index (inclusive) to the passed end index (exclusive).
    pub fn set(&mut self, start_idx: usize, end_idx: usize, info: T) {
        self.root.set(start_idx, end_idx, Rc::new(info));
    }

    /// Unset the passed syntax/format info for the passed range.
    /// The range is the passed start index (inclusive) to the passed end index (exclusive).
    pub fn unset(&mut self, start_idx: usize, end_idx: usize, info: &T) {
        self.root.unset(start_idx, end_idx, info);
    }

    /// Insert a char in the underlying text.
    pub fn insert(&mut self, idx: usize, ch: char) {
        self.root.insert(idx, ch);
    }

    /// Insert a string in the underlying text.
    pub fn insert_str(&mut self, idx: usize, string: &str) {
        self.root.insert_str(idx, string);
    }

    /// Push a char to the underlying text.
    pub fn push(&mut self, ch: char) {
        self.root.push(ch);
    }

    /// Push a string to the underlying text.
    pub fn push_str(&mut self, string: &str) {
        self.root.push_str(string);
    }

    /// Get the length of the underlying text.
    pub fn length(&self) -> usize {
        self.root.length()
    }

    /// Remove a count of characters from the underlying text starting at idx.
    pub fn remove(&mut self, idx: usize, count: usize) {
        self.root.remove(idx, count);
    }

    /// Pop a char from the underlying text.
    pub fn pop(&mut self) {
        self.remove(self.length() - 1, 1);
    }

    /// Clear the underlying text.
    /// Specify whether you want the tree to keep the formats on the root node.
    pub fn clear(&mut self, keep_formats: bool) {
        let mut new_root = Node::new_root("");

        if keep_formats {
            for info in self.root.infos() {
                new_root.add_info(Rc::clone(info));
            }
        }

        self.root = new_root;
    }

    /// Get the root node.
    pub fn get_root(&self) -> &Node<T> {
        &self.root
    }

    /// Add a change event listener.
    /// Returns the ID of the event listener (used to remove it later).
    pub fn add_listener(&mut self, l: change::Listener<T>) -> usize {
        match &mut self.listeners {
            Some(v) => v.push(l),
            None => self.listeners = Some(vec!(l)),
        };

        self.listeners.as_ref().unwrap().len() - 1
    }

    /// Remove a change event listener.
    /// Returns the listener when it could be removed.
    pub fn remove_listener(&mut self, listener_id: usize) -> Option<Listener<T>> {
        if let Some(v) = &mut self.listeners {
            Some(v.remove(listener_id))
        } else {
            None
        }
    }

    /// Get a depth first pre order iterator.
    pub fn pre_order_iter(&self) -> iterator::PreOrder<T> {
        self.root.pre_order_iter()
    }

    /// Get a leaf iterator.
    pub fn leaf_iter(&self) -> impl Iterator<Item=iterator::Item<T>> {
        self.root.leaf_iter()
    }
}

impl<T> fmt::Debug for Tree<T>
    where T: Ord + Hash + Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.root)
    }
}
