pub struct Tree<T> {
    /// The trees root node.
    root: Node<T>,
}

impl<T> Tree<T> {
    /// Create new tree.
    pub fn new(string: &str) -> Tree<T> {
        Tree {
            root: Node::new_leaf(String::from(string)),
        }
    }

    /// Load a tree.
    pub fn load() -> Tree<T> {
        // TODO Implement loading a tree from syntax/format information and text
        unimplemented!()
    }

    /// Set syntax/format info for the passed range.
    pub fn set(&mut self, start_idx: usize, end_idx: usize) {
        // TODO
        unimplemented!()
    }

    /// Insert a char in the underlying text.
    pub fn insert(&mut self, idx: usize, ch: char) {
        self.root.insert(idx, ch);
    }

    /// Insert a string in the underlying text.
    pub fn insert_str(&mut self, idx: usize, string: &str) {
        // self.root
    }

    /// Push a char to the underlying text.
    pub fn push(&mut self, ch: char) {
        // self.root.insert()

        // TODO Update last leaf node
    }

    /// Push a string to the underlying text.
    pub fn push_str(&mut self, string: &str) {
        // self.text.push_str(string);

        // TODO Update last leaf node
    }
}

pub struct Node<T> {
    /// Children of the node.
    children: Vec<Node<T>>,

    /// Object to be filled with syntax/format information.
    obj: Option<T>,

    /// Text the node (when a leaf) is holding.
    text: Option<String>,
}

impl<'a, T> Node<T> {
    /// Create new leaf node.
    pub fn new_leaf(text: String) -> Node<T> {
        Node {
            children: Vec::new(),
            obj: None,
            text: Some(text),
        }
    }

    /// Create new node.
    pub fn new(obj: T) -> Node<T> {
        Node {
            children: Vec::new(),
            obj: Some(obj),
            text: None,
        }
    }

    /// Check whether this node is a
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Add a child to this node.
    pub fn add_child(&mut self, child: Node<T>) {
        self.children.push(child);
    }

    /// Get text the node (or children) is/are holding.
    pub fn text(&self) -> String {
        if self.is_leaf() {
            self.text.as_ref().unwrap().to_string()
        } else {
            let mut result = String::with_capacity(self.length());
            for child in &self.children {
                result.push_str(&child.text());
            }
            result
        }
    }

    /// Length of the underlying text.
    pub fn length(&self) -> usize {
        if self.is_leaf() {
            self.text.as_ref().unwrap().len()
        } else {
            let mut result = 0;
            for child in &self.children {
                result += child.text.as_ref().unwrap().len();
            }
            result
        }
    }

    /// Get the object the node is holding.
    pub fn obj(&self) -> Option<&T> {
        assert!(!self.is_leaf());

        self.obj.as_ref()
    }

    /// Insert a char in the underlying text.
    pub fn insert(&mut self, idx: usize, ch: char) {
        if self.is_leaf() {
            let length = self.length();

            if idx >= length {
                panic!("Cannot insert at position {} when underlying text has length {}", idx, length);
            }

            self.text.as_mut().unwrap().insert(idx, ch);
        } else {
            let mut offset = 0;
            for child in &mut self.children {
                let length = child.length();

                if idx < offset + length {
                    child.insert(idx - offset, ch);
                    break;
                }

                offset += child.length();
            }
        }
    }

    /// Insert a string in the underlying text.
    pub fn insert_str(&mut self, idx: usize, string: &str) {
        // self.root
    }

    /// Push a char to the underlying text.
    pub fn push(&mut self, ch: char) {
        // self.root.insert()

        // TODO Update last leaf node
    }

    /// Push a string to the underlying text.
    pub fn push_str(&mut self, string: &str) {
        // self.text.push_str(string);

        // TODO Update last leaf node
    }
}

#[cfg(test)]
mod tests {
    use crate::{Tree, Node};

    #[test]
    fn insert_char_one_level() {
        let mut node: Node<()> = Node::new_leaf(String::from("Hallo"));
        node.insert(2, 'b');

        assert_eq!(node.text(), "Habllo");
    }

    #[test]
    fn insert_char_multiple_levels() {
        let mut root: Node<()> = Node::new(());
        root.add_child(Node::new_leaf(String::from("Hallo ")));
        root.add_child(Node::new_leaf(String::from("Welt")));

        root.insert(3, 'X');
        root.insert(9, 'Z');

        assert_eq!(root.text(), "HalXlo WeZlt");
    }
}
