use std::fmt;
use crate::iterator;
use std::rc::Rc;
use std::collections::{HashMap, HashSet};
use std::collections::hash_set::Iter;
use std::hash::Hash;
use std::fmt::Debug;

pub struct Node<T> {
    /// Children of the node.
    children: Vec<Node<T>>,

    /// Set to be filled with syntax/format information.
    infos: HashSet<Rc<T>>,

    /// Text the node (when a leaf) is holding.
    text: Option<String>,
}

struct AffectedNode {
    /// Affected node index.
    node_index: usize,

    /// Start of the range.
    start: usize,

    /// End of the range.
    end: usize,

    /// Whether the affected node is completely enlosed by the range.
    completely_enclosed: bool,
}

impl<T> Node<T>
    where T: Eq + Hash {
    /// Create new leaf node.
    pub fn new_leaf(text: String) -> Node<T> {
        Node {
            children: Vec::new(),
            infos: HashSet::new(),
            text: Some(text),
        }
    }

    /// Create new node.
    pub fn new() -> Node<T> {
        Node {
            children: Vec::new(),
            infos: HashSet::new(),
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

    /// Get iterator over all infos this node has.
    pub fn infos(&self) -> Iter<Rc<T>> {
        self.infos.iter()
    }

    /// Add info to the node.
    pub fn add_info(&mut self, info: Rc<T>) {
        self.infos.insert(info);
    }

    /// Remove info from the node.
    pub fn remove_info(&mut self, info: &T) {
        self.infos.remove(info);
    }

    /// Check if the node has the passed info.
    pub fn has_info(&self, info: &T) -> bool {
        self.infos.contains(info)
    }

    /// Set syntax/format info for the passed range.
    /// The range is the passed start index (inclusive) to the passed end index (exclusive).
    /// Returns a list of nodes to replace the current one in case that is needed (optional).
    pub fn set(&mut self, start_idx: usize, end_idx: usize, info: Rc<T>) -> Option<Vec<Node<T>>> {
        assert!(start_idx < end_idx);

        if self.is_leaf() {
            self.set_on_leaf(start_idx, end_idx, info)
        } else {
            self.set_on_node(start_idx, end_idx, info);
            None
        }
    }

    /// Set for a node with children.
    fn set_on_node(&mut self, start_idx: usize, end_idx: usize, info: Rc<T>) {
        // Check if affects only this node
        let length = self.length();
        if start_idx == 0 && end_idx == length {
            self.add_info(info);
        } else {
            self.set_on_node_children(start_idx, end_idx, info);
        }
    }

    /// Set on nodes children.
    fn set_on_node_children(&mut self, mut start_idx: usize, end_idx: usize, info: Rc<T>) {
        // Find out which child-node(s) is/are affected
        let mut offset = 0;
        let mut affected_children = Vec::new();
        for i in 0..self.children.len() {
            let child = &self.children[i];

            let length = child.length();

            if start_idx >= offset && start_idx <= offset + length {
                let end = if end_idx <= offset + length { end_idx - offset } else { length };

                let completely_enclosed = start_idx == offset && end == length;
                affected_children.push(AffectedNode {
                    node_index: i,
                    start: start_idx - offset,
                    end,
                    completely_enclosed,
                });

                if end_idx <= offset + length {
                    break;
                }

                start_idx = offset + length;
            }

            offset += length;
        }

        // Collect all completely enclosed child nodes.
        let mut replace_later = HashMap::new();
        let completely_enclosed: Vec<&AffectedNode> = affected_children.iter().filter(|a| a.completely_enclosed).collect();
        if completely_enclosed.len() >= 2 {
            // Build new parent node for these nodes
            let mut parent = Node::new();
            parent.add_info(Rc::clone(&info));

            // Remove all completely enclosed children from old parent and assign to the new one
            let mut removed_count = 0;
            for a in &completely_enclosed {
                parent.add_child(self.children.remove(a.node_index - removed_count));
                removed_count += 1;
            }

            // Insert new parent as child of the old parent
            self.children.insert(completely_enclosed.first().as_ref().unwrap().node_index, parent);

            // Reduce to the rest of the affected children, which have not been handled yet.
            affected_children = affected_children.into_iter().filter(|a| !a.completely_enclosed).collect();
        }

        // Set the object to the affected children.
        for i in 0..affected_children.len() {
            let affected = &affected_children[i];

            let child = &mut self.children[affected.node_index];
            if let Some(replace_with) = child.set(affected.start, affected.end, Rc::clone(&info)) {
                replace_later.insert(i, replace_with); // Replace the child node with the passed nodes later.
            }
        }

        // Replace the child nodes which need to
        for (idx, replace_with) in replace_later {
            self.children.remove(idx);

            let mut i = 0;
            for node in replace_with {
                self.children.insert(idx + i, node);
                i += 1;
            }
        }
    }

    /// Set for a leaf node.
    /// Returns a list of nodes to replace this leaf in the parent children list when
    /// there is something to replace.
    fn set_on_leaf(&mut self, start_idx: usize, end_idx: usize, info: Rc<T>) -> Option<Vec<Node<T>>> {
        let text = self.text.take().unwrap();
        let length = text.len();
        let has_infos = self.infos.len() > 0;

        assert!(start_idx <= length);
        assert!(end_idx <= length);

        if start_idx == 0 && end_idx == length {
            // Affects exactly this one leaf node
            self.add_info(info);
            self.text = Some(text);
            None
        } else if start_idx == 0 {
            // Split this leaf in two leafs
            let mut left_node = Node::new_leaf(String::from(&text[0..end_idx]));
            left_node.add_info(info);

            let right_node = Node::new_leaf(String::from(&text[end_idx..length]));

            if has_infos {
                self.add_child(left_node);
                self.add_child(right_node);
                None
            } else {
                Some(vec!(left_node, right_node))
            }
        } else if end_idx == length {
            // Split this leaf in two leafs
            let left_node = Node::new_leaf(String::from(&text[0..start_idx]));

            let mut right_node = Node::new_leaf(String::from(&text[start_idx..length]));
            right_node.add_info(info);

            if has_infos {
                self.add_child(left_node);
                self.add_child(right_node);
                None
            } else {
                Some(vec!(left_node, right_node))
            }
        } else {
            // Turn this leaf in three leafs
            let left_node = Node::new_leaf(String::from(&text[0..start_idx]));

            let mut middle_node = Node::new_leaf(String::from(&text[start_idx..end_idx]));
            middle_node.add_info(info);

            let right_node = Node::new_leaf(String::from(&text[end_idx..length]));

            if has_infos {
                self.add_child(left_node);
                self.add_child(middle_node);
                self.add_child(right_node);
                None
            } else {
                Some(vec!(left_node, middle_node, right_node))
            }
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

                if idx <= offset + length {
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

            if idx > length {
                panic!("Cannot insert at position {} when underlying text has length {}", idx, length);
            }

            self.text.as_mut().unwrap().insert_str(idx, string);
        } else {
            let mut offset = 0;
            for child in &mut self.children {
                let length = child.length();

                if idx <= offset + length {
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

impl<T> fmt::Debug for Node<T>
    where T: Ord + Hash + Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for iterator::Item { node, level } in self.pre_order_iter() {
            let mut sorted_infos: Vec<&Rc<T>> = node.infos().collect();
            sorted_infos.sort();

            writeln!(
                f,
                "{spacing}|-- '{text}'{format}",
                spacing = " ".repeat(level * 4),
                text = node.text(),
                format = format!(" {:?}", sorted_infos))?;
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
        node.insert(6, 's');
    }

    #[test]
    fn insert_char_multiple_levels() {
        let mut root: Node<()> = Node::new();
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
        let mut root: Node<()> = Node::new();
        root.add_child(Node::new_leaf(String::from("Hello ")));
        root.add_child(Node::new_leaf(String::from("World")));

        root.insert_str(3, "XXXX");
        root.insert_str(12, "ZZZZ");

        assert_eq!(root.text(), "HelXXXXlo WoZZZZrld");
    }

    #[test]
    fn push_string() {
        let mut root: Node<()> = Node::new();

        let child1: Node<()> = Node::new_leaf(String::from("Hello "));
        root.add_child(child1);

        let mut child2: Node<()> = Node::new();
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
        let mut root: Node<()> = Node::new();

        let mut child1: Node<()> = Node::new();
        let subchild1: Node<()> = Node::new_leaf(String::from("Hel"));
        let subchild2: Node<()> = Node::new_leaf(String::from("lo "));
        child1.add_child(subchild1);
        child1.add_child(subchild2);
        root.add_child(child1);

        let mut child2: Node<()> = Node::new();
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
