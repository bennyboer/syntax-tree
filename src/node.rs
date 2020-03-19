use std::fmt;
use crate::iterator;

pub struct Node<T> {
    /// Children of the node.
    children: Vec<Node<T>>,

    /// Object to be filled with syntax/format information.
    obj: Option<T>,

    /// Text the node (when a leaf) is holding.
    text: Option<String>,
}

impl<T> Node<T> {
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
                result += child.length();
            }
            result
        }
    }

    /// Get the object the node is holding.
    pub fn obj(&self) -> Option<&T> {
        assert!(!self.is_leaf());

        self.obj.as_ref()
    }

    /// Set syntax/format info for the passed range.
    /// The range is the passed start index (inclusive) to the passed end index (exclusive).
    pub fn set(&mut self, start_idx: usize, end_idx: usize, obj: T) {
        assert!(start_idx < end_idx);

        if self.is_leaf() {
            self.set_leaf(start_idx, end_idx, obj);
        } else {}
    }

    /// Set for a leaf node.
    fn set_leaf(&mut self, start_idx: usize, end_idx: usize, obj: T) {
        let text = self.text.take().unwrap();
        let length = text.len();

        assert!(start_idx <= length);
        assert!(end_idx <= length);

        if start_idx == 0 && end_idx == length {
            // Affects exactly this one leaf -> Turn into a node
            self.obj = Some(obj);
            self.add_child(Node::new_leaf(text));
        } else if start_idx == 0 {
            // Turn into node with two children
            let left_str = String::from(&text[0..end_idx]);
            let right_str = String::from(&text[end_idx..length]);

            let mut obj_node = Node::new(obj);
            obj_node.add_child(Node::new_leaf(left_str));

            self.add_child(obj_node);
            self.add_child(Node::new_leaf(right_str));
        } else if end_idx == length {
            // Turn into node with two children
            let left_str = String::from(&text[0..start_idx]);
            let right_str = String::from(&text[start_idx..length]);

            let mut obj_node = Node::new(obj);
            obj_node.add_child(Node::new_leaf(right_str));

            self.add_child(Node::new_leaf(left_str));
            self.add_child(obj_node);
        } else {
            // Turn into node with three children
            let left_str = String::from(&text[0..start_idx]);
            let middle_str = String::from(&text[start_idx..end_idx]);
            let right_str = String::from(&text[end_idx..length]);

            let mut obj_node = Node::new(obj);
            obj_node.add_child(Node::new_leaf(middle_str));

            self.add_child(Node::new_leaf(left_str));
            self.add_child(obj_node);
            self.add_child(Node::new_leaf(right_str));
        }
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
        if self.is_leaf() {
            let length = self.length();

            if idx >= length {
                panic!("Cannot insert at position {} when underlying text has length {}", idx, length);
            }

            self.text.as_mut().unwrap().insert_str(idx, string);
        } else {
            let mut offset = 0;
            for child in &mut self.children {
                let length = child.length();

                if idx < offset + length {
                    child.insert_str(idx - offset, string);
                    break;
                }

                offset += child.length();
            }
        }
    }

    /// Push a char to the underlying text.
    pub fn push(&mut self, ch: char) {
        if self.is_leaf() {
            self.text.as_mut().unwrap().push(ch);
        } else {
            self.children.last_mut().unwrap().push(ch);
        }
    }

    /// Push a string to the underlying text.
    pub fn push_str(&mut self, string: &str) {
        if self.is_leaf() {
            self.text.as_mut().unwrap().push_str(string);
        } else {
            self.children.last_mut().unwrap().push_str(string);
        }
    }

    /// Get the count of children under this node.
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Get a slice of all children under this node.
    pub fn children(&self) -> &[Node<T>] {
        &self.children[..]
    }

    /// Get a depth first pre order iterator.
    pub fn pre_order_iter(&self) -> iterator::PreOrder<T> {
        iterator::PreOrder::new(self)
    }

    /// Get a leaf iterator.
    pub fn leaf_iter(&self) -> impl Iterator<Item=iterator::Item<T>> {
        self.pre_order_iter().filter(|item| item.node.is_leaf())
    }
}

impl<T> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for iterator::Item { node, level } in self.pre_order_iter() {
            if node.is_leaf() {
                writeln!(f, "{spacing}|-- '{text}'", spacing = " ".repeat(level * 4), text = node.text())?;
            } else {
                writeln!(f, "{spacing}|---o ('{text}')", spacing = " ".repeat(level * 4), text = node.text())?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Node;

    #[test]
    fn insert_char_one_level() {
        let mut node: Node<()> = Node::new_leaf(String::from("Hello"));
        node.insert(2, 'b');

        assert_eq!(node.text(), "Hebllo");
    }

    #[test]
    #[should_panic]
    fn insert_char_panic() {
        let mut node: Node<()> = Node::new_leaf(String::from("Hello"));
        node.insert(233, 's');
    }

    #[test]
    fn insert_char_multiple_levels() {
        let mut root: Node<()> = Node::new(());
        root.add_child(Node::new_leaf(String::from("Hello ")));
        root.add_child(Node::new_leaf(String::from("World")));

        root.insert(3, 'X');
        root.insert(9, 'Z');

        assert_eq!(root.text(), "HelXlo WoZrld");
    }

    #[test]
    fn insert_string_one_level() {
        let mut node: Node<()> = Node::new_leaf(String::from("Hello"));
        node.insert_str(3, "TEST");

        assert_eq!(node.text(), "HelTESTlo");
    }

    #[test]
    #[should_panic]
    fn insert_string_panic() {
        let mut node: Node<()> = Node::new_leaf(String::from("Hello"));
        node.insert_str(233, "wefewf");
    }

    #[test]
    fn insert_string_multiple_levels() {
        let mut root: Node<()> = Node::new(());
        root.add_child(Node::new_leaf(String::from("Hello ")));
        root.add_child(Node::new_leaf(String::from("World")));

        root.insert_str(3, "XXXX");
        root.insert_str(12, "ZZZZ");

        assert_eq!(root.text(), "HelXXXXlo WoZZZZrld");
    }

    #[test]
    fn push_string() {
        let mut root: Node<()> = Node::new(());

        let child1: Node<()> = Node::new_leaf(String::from("Hello "));
        root.add_child(child1);

        let mut child2: Node<()> = Node::new(());
        let subchild1: Node<()> = Node::new_leaf(String::from("Wor"));
        let subchild2: Node<()> = Node::new_leaf(String::from("ld"));
        child2.add_child(subchild1);
        child2.add_child(subchild2);
        root.add_child(child2);

        root.push_str("! I am a pushed string!");

        assert_eq!(root.text(), "Hello World! I am a pushed string!");
    }

    #[test]
    fn push_char() {
        let mut root: Node<()> = Node::new(());

        let mut child1: Node<()> = Node::new(());
        let subchild1: Node<()> = Node::new_leaf(String::from("Hel"));
        let subchild2: Node<()> = Node::new_leaf(String::from("lo "));
        child1.add_child(subchild1);
        child1.add_child(subchild2);
        root.add_child(child1);

        let mut child2: Node<()> = Node::new(());
        let subchild1: Node<()> = Node::new_leaf(String::from("Wor"));
        let subchild2: Node<()> = Node::new_leaf(String::from("ld"));
        let subchild3: Node<()> = Node::new_leaf(String::from("!"));
        child2.add_child(subchild1);
        child2.add_child(subchild2);
        child2.add_child(subchild3);
        root.add_child(child2);

        root.push('!');

        assert_eq!(root.text(), "Hello World!!");
    }
}
