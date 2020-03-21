use std::fmt;
use crate::iterator;
use std::rc::Rc;
use std::collections::HashSet;
use std::collections::hash_set::Iter;
use std::hash::Hash;
use std::fmt::Debug;

pub struct Node<T> {
    /// Children of the node.
    children: Option<Vec<Node<T>>>,

    /// Set to be filled with syntax/format information.
    infos: HashSet<Rc<T>>,

    /// Text the node (when a leaf) is holding.
    text: Option<String>,

    /// Whether this node is the root node.
    root: bool,
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
            children: None,
            infos: HashSet::new(),
            text: Some(text),
            root: false,
        }
    }

    /// Create new node.
    pub fn new() -> Node<T> {
        Node {
            children: None,
            infos: HashSet::new(),
            text: None,
            root: false,
        }
    }

    /// Create new root node.
    pub fn new_root(string: &str) -> Node<T> {
        Node {
            children: None,
            infos: HashSet::new(),
            text: Some(String::from(string)),
            root: true,
        }
    }

    /// Check whether this node is a
    pub fn is_leaf(&self) -> bool {
        match self.children.as_ref() {
            Some(v) => v.is_empty(),
            None => true
        }
    }

    /// Add a child to this node.
    pub fn add_child(&mut self, child: Node<T>) {
        if self.children.is_none() {
            self.children = Some(Vec::new());
        }

        self.children.as_mut().unwrap().push(child);
    }

    /// Get text the node (or children) is/are holding.
    pub fn text(&self) -> String {
        if self.is_leaf() {
            self.text.as_ref().unwrap().to_string()
        } else {
            let mut result = String::with_capacity(self.length());
            for child in self.children.as_ref().unwrap() {
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
            for child in self.children.as_ref().unwrap() {
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
    /// Specify recurse whether the info should be removed from children as well.
    /// Since a node can end up useless without info, it might have to be replaced
    /// by its children which are then returned by this method.
    pub fn remove_info(&mut self, start_idx: usize, end_idx: usize, info: &T, recurse: bool) -> Option<Vec<Node<T>>> {
        self.infos.remove(info);

        if recurse && !self.is_leaf() {
            let children = self.children.as_mut().unwrap();

            let mut offset = 0;
            let mut replace_later = Vec::new();
            for i in 0..children.len() {
                let child = &mut children[i];
                let length = child.length();
                let ranges_intersect = offset < end_idx && start_idx < offset + length;

                if ranges_intersect || (child.is_leaf() && child.infos.len() == 0) {
                    if let Some(v) = child.remove_info(start_idx, end_idx, info, recurse) {
                        replace_later.push((i, v));
                    }
                }

                offset += length;
            }

            // Find and process single-item replace later nodes which consist of one
            // unformatted leaf. If they are adjacent, they can be merged.
            // Handle the others as usual by replacing the old child with its children.
            let mut replace_later_single_unformatted_leafs = Vec::new();
            let mut removed = 0;
            for (idx, mut nodes) in replace_later.into_iter() {
                children.remove(idx - removed);
                removed += 1;

                if nodes.len() == 1 && nodes.first().unwrap().is_leaf() && nodes.first().unwrap().infos.len() == 0 {
                    // Is only one unformatted leaf
                    replace_later_single_unformatted_leafs.push((idx, nodes.remove(0)));
                } else {
                    // Replace the old node by its children.
                    let mut i = 0;
                    for node in nodes {
                        children.insert(idx + i, node);
                        i += 1;
                    }
                }
            }

            if !replace_later_single_unformatted_leafs.is_empty() {
                // Collect and merge adjacent unformatted leafs.
                let (mut start_idx, first_node) = replace_later_single_unformatted_leafs.remove(0);
                let mut last_idx = start_idx;
                let mut collector = vec!((last_idx, first_node));

                let mut to_merge = Vec::new();
                let mut to_insert = Vec::new();
                for (idx, node) in replace_later_single_unformatted_leafs {
                    if idx == last_idx + 1 {
                        collector.push((idx, node));
                    } else {
                        if collector.len() > 1 {
                            to_merge.push((start_idx, collector));
                        } else {
                            to_insert.push(collector.remove(0));
                        }
                        start_idx = last_idx;
                        collector = vec!((idx, node));
                    }

                    last_idx = idx;
                }
                if collector.len() >= 2 {
                    to_merge.push((start_idx, collector));
                } else {
                    to_insert.push(collector.remove(0));
                }

                // Merge adjacent unformatted leafs.
                for (idx, nodes) in to_merge {
                    let mut string = String::new();
                    for (_, n) in nodes {
                        string.push_str(n.text.as_ref().unwrap());
                    }

                    children.insert(idx, Node::new_leaf(string));
                }

                // Insert remaining
                for (idx, node) in to_insert {
                    children.insert(idx, node);
                }
            }

            // Check if we have only one leaf child without info left
            if children.len() == 1 && children.first().unwrap().is_leaf() {
                // Turn this node into a leaf
                let n = children.remove(0);
                self.children = None;

                self.text = Some(n.text.unwrap());
                for info in n.infos.into_iter() {
                    self.add_info(info);
                }
            }
        }

        if self.infos.len() == 0 && !self.root {
            if self.is_leaf() {
                Some(vec!(Node::new_leaf(self.text.take().unwrap())))
            } else {
                // This node has no use -> replace with it's children
                Some(self.children.take().unwrap())
            }
        } else {
            None
        }
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

    /// Unset the passed syntax/format info for the passed range.
    /// The range is the passed start index (inclusive) to the passed end index (exclusive).
    pub fn unset(&mut self, start_idx: usize, end_idx: usize, info: &T) {
        assert!(start_idx < end_idx);

        if let Some(v) = self.remove_info(start_idx, end_idx, info, true) {
            self.children = Some(v);
        }
    }

    /// Set for a node with children.
    fn set_on_node(&mut self, start_idx: usize, end_idx: usize, info: Rc<T>) {
        // Check if affects only this node
        let length = self.length();
        if start_idx == 0 && end_idx == length {
            // Remove info in children -> now unnecessary
            if let Some(v) = self.remove_info(0, length, &info, true) {
                self.children = Some(v);
            }
            self.add_info(info);
        } else {
            self.set_on_node_children(start_idx, end_idx, info);
        }
    }

    /// Set on nodes children.
    fn set_on_node_children(&mut self, mut start_idx: usize, end_idx: usize, info: Rc<T>) {
        let children = self.children.as_mut().unwrap();
        let child_count = children.len();

        // Find out which child-node(s) is/are affected
        let mut offset = 0;
        let mut affected_children = Vec::new();
        for i in 0..child_count {
            let child = &children[i];

            let length = child.length();

            if start_idx >= offset && start_idx < offset + length {
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
        let completely_enclosed: Vec<&AffectedNode> = affected_children.iter().filter(|a| a.completely_enclosed).collect();
        if completely_enclosed.len() >= 2 {
            // Build new parent node for these nodes
            let mut parent = Node::new();
            parent.add_info(Rc::clone(&info));

            // Remove all completely enclosed children from old parent and assign to the new one
            let mut removed_count = 0;
            for a in &completely_enclosed {
                parent.add_child(children.remove(a.node_index - removed_count));
                removed_count += 1;
            }

            // Insert new parent as child of the old parent
            let insert_idx = completely_enclosed.first().as_ref().unwrap().node_index;
            children.insert(insert_idx, parent);

            // Reduce to the rest of the affected children, which have not been handled yet.
            affected_children = affected_children.into_iter()
                .filter(|a| !a.completely_enclosed)
                .map(|mut a| {
                    if a.node_index > insert_idx {
                        a.node_index -= removed_count;
                    }

                    a
                }).collect();
        }

        // Set the object to the affected children.
        let mut replace_later = Vec::new();
        for i in 0..affected_children.len() {
            let affected = &affected_children[i];

            let child = &mut children[affected.node_index];
            if let Some(replace_with) = child.set(affected.start, affected.end, Rc::clone(&info)) {
                replace_later.push((affected.node_index, replace_with)); // Replace the child node with the passed nodes later.
            }
        }

        // Replace the child nodes which need to
        let mut added = 0;
        for (idx, replace_with) in replace_later {
            children.remove(idx);

            for node in replace_with {
                children.insert(idx + added, node);
                added += 1;
            }
        }

        self.regroup_neighbors();
    }

    /// Check if the passed node has the same formats than this node.
    fn has_same_infos(&self, other_node: &Node<T>) -> bool {
        if self.infos.len() != other_node.infos.len() {
            return false;
        }

        for info in &other_node.infos {
            if !self.has_info(info) {
                return false;
            }
        }

        return true;
    }

    /// Regroup neighboring nodes with similar syntax/format info.
    fn regroup_neighbors(&mut self) {
        if let Some((info, indices)) = self.find_max_similar_neighbors() {
            // Create new parent node for the similar nodes
            let mut parent = Node::new();

            let insert_idx = indices[0];

            let mut removed = 0;
            for idx in indices {
                let mut child = self.children.as_mut().unwrap().remove(idx - removed);
                match child.remove_info(0, child.length(), &info, false) {
                    Some(v) => {
                        for n in v {
                            parent.add_child(n);
                        }
                    }
                    None => parent.add_child(child),
                }
                removed += 1;
            }

            parent.add_info(info);

            self.children.as_mut().unwrap().insert(insert_idx, parent);

            // Check if we have only one child left with the same syntax/format info as this node
            if self.child_count() == 1 {
                if self.has_same_infos(&self.children.as_ref().unwrap()[0]) {
                    // Merge node with child
                    let mut child = self.children.as_mut().unwrap().remove(0);
                    self.children = Some(child.children.take().unwrap());
                }
            }
        }
    }

    /// Find the maximum similar neighbors in the nodes children.
    fn find_max_similar_neighbors<'a>(&self) -> Option<(Rc<T>, Vec<usize>)> {
        let children = self.children.as_ref().unwrap();
        let length = children.len();

        let mut max_result: Option<(Rc<T>, Vec<usize>)> = None;
        for i in 0..length {
            let child = &children[i];

            for info in &child.infos {
                let mut similar = vec!(i);
                for o in i + 1..length {
                    let other_child = &children[o];
                    if other_child.has_info(info) {
                        similar.push(o);
                    } else {
                        break;
                    }
                }

                if similar.len() > 1 && (max_result.is_none() || max_result.as_ref().unwrap().1.len() < similar.len()) {
                    max_result = Some((Rc::clone(info), similar));
                }
            }
        }

        max_result
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

            if has_infos || self.root {
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

            if has_infos || self.root {
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

            if has_infos || self.root {
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
            for child in self.children.as_mut().unwrap() {
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
            for child in self.children.as_mut().unwrap() {
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
            self.children.as_mut().unwrap().last_mut().unwrap().push(ch);
        }
    }

    /// Push a string to the underlying text.
    pub fn push_str(&mut self, string: &str) {
        if self.is_leaf() {
            self.text.as_mut().unwrap().push_str(string);
        } else {
            self.children.as_mut().unwrap().last_mut().unwrap().push_str(string);
        }
    }

    /// Get the count of children under this node.
    pub fn child_count(&self) -> usize {
        match self.children.as_ref() {
            Some(v) => v.len(),
            None => 0,
        }
    }

    /// Get a slice of all children under this node.
    pub fn children(&self) -> &[Node<T>] {
        &self.children.as_ref().unwrap()[..]
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
