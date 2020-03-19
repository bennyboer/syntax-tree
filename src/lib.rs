pub struct Tree<'a, T> {
    /// The trees root node.
    root: Node<'a, T>,

    /// Total text the tree is holding.
    text: String,
}

pub struct Node<'a, T> {
    /// Children of the node.
    children: Vec<Node<'a, T>>,

    /// Object to be filled with syntax/format information.
    obj: Option<T>,

    /// Text the node (when a leaf) is holding.
    text: Option<&'a String>,
}

impl<T> Node<'_, T> {
    /// Check whether this node is a
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Get text the node (when a leaf) is holding.
    pub fn text(&self) -> &String {
        assert!(self.is_leaf());

        self.text.unwrap()
    }

    /// Get the object the node is holding.
    pub fn obj(&self) -> Option<&T> {
        assert!(!self.is_leaf());

        self.obj.as_ref()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
