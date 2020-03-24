use std::fmt;
use crate::{iterator, change};
use std::rc::Rc;
use std::collections::HashSet;
use std::collections::hash_set::Iter;
use std::hash::Hash;
use std::fmt::Debug;
use uuid::Uuid;

pub struct Node<T> {
    /// ID uniquely identifying the node.
    id: String,

    /// Children of the node.
    children: Option<Vec<Node<T>>>,

    /// Set to be filled with syntax/format information.
    infos: HashSet<Rc<T>>,

    /// Text the node (when a leaf) is holding.
    text: Option<String>,

    /// Whether this node is the root node.
    root: bool,

    /// Change event listener reference.
    listener: Option<Rc<change::Listener<T>>>,
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
            id: Uuid::new_v4().to_string(),
            children: None,
            infos: HashSet::new(),
            text: Some(text),
            root: false,
            listener: None,
        }
    }

    /// Create new node.
    pub fn new() -> Node<T> {
        Node {
            id: Uuid::new_v4().to_string(),
            children: None,
            infos: HashSet::new(),
            text: None,
            root: false,
            listener: None,
        }
    }

    /// Create new root node.
    pub fn new_root(string: &str) -> Node<T> {
        Node {
            id: Uuid::new_v4().to_string(),
            children: None,
            infos: HashSet::new(),
            text: Some(String::from(string)),
            root: true,
            listener: None,
        }
    }

    /// Get the ID of the node.
    pub fn id(&self) -> &String {
        &self.id
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
        self.emit_event(change::Event::NodeAdded { parent: &self, added_idx: self.child_count() - 1 });
    }

    /// Get text the node (or children) is/are holding.
    pub fn text(&self) -> String {
        if self.is_leaf() {
            match self.text.as_ref() {
                Some(v) => v.to_string(),
                None => {
                    println!("WARNING: Leaf does not have text");
                    "".to_string()
                }
            }
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

    /// Clear all infos from this node without recursion in children.
    pub fn clear_infos(&mut self) {
        self.infos.clear();
    }

    /// Remove info from the node.
    /// Specify recurse whether the info should be removed from children as well.
    /// Since a node can end up useless without info, it might have to be replaced
    /// by its children which are then returned by this method.
    pub fn remove_info(&mut self, start_idx: usize, end_idx: usize, info: Rc<T>, recurse: bool) -> Option<Vec<Node<T>>> {
        let mut set_later = Vec::new();

        if self.is_leaf() {
            if self.infos.remove(&info) {
                self.emit_event(change::Event::InfosChanged { node: &self });

                let length = self.length();

                if start_idx == 0 && end_idx == length {
                    // Intersects fully -> Do nothing
                } else if start_idx == 0 {
                    // Intersects only in the beginning of the child node -> Split and remove info from left node.
                    set_later.push((vec!((end_idx, length)), vec!(info)));
                } else if end_idx == length {
                    // Intersects only in the end of the child node -> Split and remove info from the right node.
                    set_later.push((vec!((0, start_idx)), vec!(info)));
                } else {
                    // Intersects in the middle of the child node -> Split and remove info from the middle node.
                    set_later.push((vec!((0, start_idx), (end_idx, length)), vec!(info)));
                }
            }
        } else if recurse {
            if self.infos.remove(&info) {
                self.emit_event(change::Event::InfosChanged { node: &self });
            }

            let mut offset = 0;
            let mut replace_later = Vec::new();
            for i in 0..self.child_count() {
                let child = &mut self.children.as_mut().unwrap()[i];
                let length = child.length();
                let ranges_intersect = offset < end_idx && start_idx < offset + length;

                if ranges_intersect {
                    let start = if start_idx > offset { start_idx - offset } else { 0 };
                    let end = if end_idx - offset > length { length } else { end_idx - offset };

                    let mut old_infos = Vec::new();
                    for old_info in child.infos() {
                        old_infos.push(Rc::clone(old_info));
                    }

                    if let Some(v) = child.remove_info(start, end, Rc::clone(&info), recurse) {
                        replace_later.push((i, v));
                    }

                    if old_infos.len() > 0 {
                        if start == 0 && end == length {
                            // Intersects fully -> Just remove info from child node
                        } else if start == 0 {
                            // Intersects only in the beginning of the child node -> Split and remove info from left node.
                            set_later.push((vec!((offset + end, offset + length)), old_infos));
                        } else if end == length {
                            // Intersects only in the end of the child node -> Split and remove info from the right node.
                            set_later.push((vec!((offset, start + offset)), old_infos));
                        } else {
                            // Intersects in the middle of the child node -> Split and remove info from the middle node.
                            set_later.push((vec!((offset, start + offset), (offset + end, offset + length)), old_infos));
                        }
                    }
                } else if child.is_leaf() {
                    let mut new_leaf = Node::new_leaf(child.text.take().unwrap());
                    new_leaf.give_listener(&self.listener);

                    for old_info in child.infos() {
                        new_leaf.add_info(Rc::clone(old_info));
                    }

                    replace_later.push((i, vec!(new_leaf)));
                } else {
                    replace_later.push((i, child.children.take().unwrap()));
                }

                offset += length;
            }

            // Find and process single-item replace later nodes which consist of one
            // unformatted leaf. If they are adjacent, they can be merged.
            // Handle the others as usual by replacing the old child with its children.
            let mut replace_later_single_unformatted_leafs = Vec::new();
            let mut to_insert = Vec::new();
            let mut removed = 0;
            let mut additional_children = 0;
            for (idx, nodes) in replace_later.into_iter() {
                self.children.as_mut().unwrap().remove(idx - removed);
                self.emit_event(change::Event::NodeRemoved { parent: &self, removed_idx: idx - removed });
                removed += 1;

                // Replace the old node by its children.
                let add_children = nodes.len() - 1;

                let mut i = 0;
                for node in nodes {
                    if node.is_leaf() && node.infos.len() == 0 {
                        replace_later_single_unformatted_leafs.push((idx + i + additional_children, node));
                    } else {
                        to_insert.push((idx + i + additional_children, node));
                    }
                    i += 1;
                }

                additional_children += add_children;
            }

            if !replace_later_single_unformatted_leafs.is_empty() {
                // Collect and merge adjacent unformatted leafs.
                let (mut start_idx, first_node) = replace_later_single_unformatted_leafs.remove(0);
                let mut last_idx = start_idx;
                let mut collector = vec!((last_idx, first_node));

                let mut reduced_count = 0;

                let mut to_merge = Vec::new();
                for (idx, node) in replace_later_single_unformatted_leafs {
                    if idx == last_idx + 1 {
                        collector.push((idx, node));
                    } else {
                        if collector.len() > 1 {
                            reduced_count += collector.len() - 1;
                            to_merge.push((start_idx, collector));
                        } else {
                            let (idx, nodes) = collector.remove(0);
                            to_insert.push((idx - reduced_count, nodes));
                        }
                        start_idx = idx;
                        collector = vec!((idx, node));
                    }

                    last_idx = idx;
                }
                if collector.len() >= 2 {
                    to_merge.push((start_idx, collector));
                } else {
                    let (idx, nodes) = collector.remove(0);
                    to_insert.push((idx - reduced_count, nodes));
                }

                // Merge adjacent unformatted leafs.
                for (idx, nodes) in to_merge {
                    let reduces_by = nodes.len() - 1;
                    let mut string = String::new();
                    for (_, n) in nodes {
                        string.push_str(n.text.as_ref().unwrap());
                    }

                    let mut new_leaf = Node::new_leaf(string);
                    new_leaf.give_listener(&self.listener);

                    to_insert.push((idx - reduced_count, new_leaf));
                    reduced_count += reduces_by;
                }
            }

            // Insert remaining
            to_insert.sort_by_key(|(idx, _)| *idx);
            let mut child_count = self.child_count();
            for (idx, node) in to_insert {
                if idx >= child_count {
                    &mut self.children.as_mut().unwrap().push(node);
                    self.emit_event(change::Event::NodeAdded { parent: &self, added_idx: child_count });
                } else {
                    &mut self.children.as_mut().unwrap().insert(idx, node);
                    self.emit_event(change::Event::NodeAdded { parent: &self, added_idx: idx });
                }

                child_count += 1;
            }

            // Check if we have only one leaf child without info left
            if self.child_count() == 1 && self.children.as_ref().unwrap().first().unwrap().is_leaf() {
                // Turn this node into a leaf
                let mut n = self.children.as_mut().unwrap().remove(0);
                self.children = None;
                self.text = Some(n.text.take().unwrap());
                self.emit_event(change::Event::NodeRemoved { parent: &self, removed_idx: 0 });

                let mut infos_added = false;
                for info in n.infos.into_iter() {
                    self.add_info(info);
                    infos_added = true;
                }

                if infos_added {
                    self.emit_event(change::Event::InfosChanged { node: &self });
                }
            }

            if self.child_count() > 1 {
                self.regroup_neighbors();
            }
        }

        for (ranges, old_infos) in set_later {
            for (a, b) in ranges {
                for old_info in &old_infos {
                    if let Some(v) = self.set(a, b, Rc::clone(old_info)) {
                        for n in v {
                            self.add_child(n);
                        }
                    }
                }
            }
        }

        if self.infos.len() == 0 && !self.root {
            if self.is_leaf() {
                let mut new_leaf = Node::new_leaf(self.text.take().unwrap());
                new_leaf.give_listener(&self.listener);

                Some(vec!(new_leaf))
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

        if self.has_info(&info) {
            return None;
        }

        if self.is_leaf() {
            self.set_on_leaf(start_idx, end_idx, info)
        } else {
            self.set_on_node(start_idx, end_idx, info);
            None
        }
    }

    /// Unset the passed syntax/format info for the passed range.
    /// The range is the passed start index (inclusive) to the passed end index (exclusive).
    pub fn unset(&mut self, start_idx: usize, end_idx: usize, info: Rc<T>) {
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
            if let Some(v) = self.remove_info(0, length, Rc::clone(&info), true) {
                self.children = Some(v);
            }

            self.add_info(info);
            self.emit_event(change::Event::InfosChanged { node: &self });
        } else {
            self.set_on_node_children(start_idx, end_idx, Rc::clone(&info));
        }
    }

    /// Set on nodes children.
    fn set_on_node_children(&mut self, mut start_idx: usize, end_idx: usize, info: Rc<T>) {
        // Find out which child-node(s) is/are affected
        let mut offset = 0;
        let mut affected_children = Vec::new();
        for i in 0..self.child_count() {
            let child = &self.children.as_mut().unwrap()[i];
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
            parent.give_listener(&self.listener);
            parent.add_info(Rc::clone(&info));

            // Remove all completely enclosed children from old parent and assign to the new one
            let mut removed_count = 0;
            for a in &completely_enclosed {
                let removed_child = self.children.as_mut().unwrap().remove(a.node_index - removed_count);
                self.emit_event(change::Event::NodeRemoved { parent: &self, removed_idx: a.node_index - removed_count });

                parent.add_child(removed_child);
                removed_count += 1;
            }

            // Insert new parent as child of the old parent
            let insert_idx = completely_enclosed.first().as_ref().unwrap().node_index;
            self.children.as_mut().unwrap().insert(insert_idx, parent);
            self.emit_event(change::Event::NodeAdded { parent: &self, added_idx: insert_idx });

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

            let child = &mut self.children.as_mut().unwrap()[affected.node_index];
            if let Some(replace_with) = child.set(affected.start, affected.end, Rc::clone(&info)) {
                replace_later.push((affected.node_index, replace_with)); // Replace the child node with the passed nodes later.
            }
        }

        // Replace the child nodes which need to
        let mut added = 0;
        for (idx, replace_with) in replace_later {
            self.children.as_mut().unwrap().remove(idx);
            self.emit_event(change::Event::NodeRemoved { parent: &self, removed_idx: idx });

            for node in replace_with {
                self.children.as_mut().unwrap().insert(idx + added, node);
                self.emit_event(change::Event::NodeAdded { parent: &self, added_idx: idx + added });
                added += 1;
            }
        }

        self.regroup_neighbors();
    }

    /// Regroup neighboring nodes with similar syntax/format info.
    fn regroup_neighbors(&mut self) {
        if let Some((info, indices)) = self.find_max_similar_neighbors() {
            // Create new parent node for the similar nodes
            let mut parent = Node::new();
            parent.give_listener(&self.listener);

            let insert_idx = indices[0];

            let mut removed = 0;
            let mut to_add = Vec::new();
            for idx in indices {
                let mut child = self.children.as_mut().unwrap().remove(idx - removed);
                self.emit_event(change::Event::NodeRemoved { parent: &self, removed_idx: idx - removed });

                match child.remove_info(0, child.length(), Rc::clone(&info), true) {
                    Some(v) => {
                        for n in v {
                            to_add.push(n);
                        }
                    }
                    None => to_add.push(child),
                }
                removed += 1;
            }

            if to_add.iter().all(|n| n.infos.len() == 0) {
                // Merge all children
                let mut string = String::new();
                for mut n in to_add {
                    string.push_str(&n.text.take().unwrap());
                }
                parent.text = Some(string);
            } else {
                for n in to_add {
                    parent.add_child(n);
                }
                parent.regroup_neighbors();
            }

            parent.add_info(info);

            self.children.as_mut().unwrap().insert(insert_idx, parent);
            self.emit_event(change::Event::NodeAdded { parent: &self, added_idx: insert_idx });

            // Check if we have only one child left with the same syntax/format info as this node
            if self.child_count() == 1 {
                // Merge node with child
                let mut child = self.children.as_mut().unwrap().remove(0);
                self.emit_event(change::Event::NodeRemoved { parent: &self, removed_idx: 0 });

                self.children = Some(child.children.take().unwrap());
                for i in 0..self.child_count() {
                    self.emit_event(change::Event::NodeAdded { parent: &self, added_idx: i });
                }

                if child.infos.len() > 0 {
                    for i in child.infos {
                        self.add_info(i);
                    }
                    self.emit_event(change::Event::InfosChanged { node: &self });
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
            self.emit_event(change::Event::InfosChanged { node: &self });
            None
        } else if start_idx == 0 {
            // Split this leaf in two leafs
            let mut left_node = Node::new_leaf(String::from(&text[0..end_idx]));
            left_node.give_listener(&self.listener);
            left_node.add_info(info);

            let mut right_node = Node::new_leaf(String::from(&text[end_idx..length]));
            right_node.give_listener(&self.listener);

            if has_infos || self.root {
                self.add_child(left_node);
                self.add_child(right_node);
                None
            } else {
                Some(vec!(left_node, right_node))
            }
        } else if end_idx == length {
            // Split this leaf in two leafs
            let mut left_node = Node::new_leaf(String::from(&text[0..start_idx]));
            left_node.give_listener(&self.listener);

            let mut right_node = Node::new_leaf(String::from(&text[start_idx..length]));
            right_node.give_listener(&self.listener);
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
            let mut left_node = Node::new_leaf(String::from(&text[0..start_idx]));
            left_node.give_listener(&self.listener);

            let mut middle_node = Node::new_leaf(String::from(&text[start_idx..end_idx]));
            middle_node.give_listener(&self.listener);
            middle_node.add_info(info);

            let mut right_node = Node::new_leaf(String::from(&text[end_idx..length]));
            right_node.give_listener(&self.listener);

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
            self.emit_event(change::Event::TextChanged { node: &self });
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
            self.emit_event(change::Event::TextChanged { node: &self });
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
            self.emit_event(change::Event::TextChanged { node: &self });
        } else {
            self.children.as_mut().unwrap().last_mut().unwrap().push(ch);
        }
    }

    /// Push a string to the underlying text.
    pub fn push_str(&mut self, string: &str) {
        if self.is_leaf() {
            self.text.as_mut().unwrap().push_str(string);
            self.emit_event(change::Event::TextChanged { node: &self });
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

    /// Remove a count of characters from the underlying text starting at idx.
    /// Removing a character might lead to having an empty leaf left.
    /// Method will return boolean tuple with two elements (bool, bool).
    /// The first boolean will determine whether the node is unnecessary now,
    /// the second boolean determined whether parent may need to regroup its children.
    pub fn remove(&mut self, mut idx: usize, mut count: usize) -> (bool, bool) {
        let length = self.length();

        if self.is_leaf() {
            assert!(idx + count <= length);

            self.text.as_mut().unwrap().replace_range(idx..idx + count, "");
            if self.length() > 0 {
                self.emit_event(change::Event::TextChanged { node: &self });
            }
        } else {
            // Remove from affected children
            let mut offset = 0;
            let mut remove_later = Vec::new();
            let mut may_need_regroup = false;
            for i in 0..self.child_count() {
                let child = &mut self.children.as_mut().unwrap()[i];
                let length = child.length();

                if idx >= offset && idx < offset + length {
                    // Affects child
                    let max_end = offset + length;
                    let end = if idx + count < max_end { idx + count } else { max_end };

                    let remove_count = end - idx;
                    let (unnecessary, needs_regroup) = child.remove(idx - offset, remove_count);
                    if unnecessary {
                        remove_later.push(i);
                    }
                    may_need_regroup = may_need_regroup || needs_regroup;

                    if idx + count <= max_end {
                        break; // Next child is not affected
                    }

                    idx += remove_count;
                    count -= remove_count;
                }

                offset += length;
            }

            // Remove now unnecessary children
            let mut removed = 0;
            for i in remove_later {
                self.children.as_mut().unwrap().remove(i - removed);
                self.emit_event(change::Event::NodeRemoved { parent: &self, removed_idx: i - removed });
                removed += 1;
            }

            // Check if having only one child left
            if self.child_count() == 1 {
                let mut child = self.children.as_mut().unwrap().remove(0);
                self.children = None;
                self.text = Some(child.text.take().unwrap());
                self.emit_event(change::Event::NodeRemoved { parent: &self, removed_idx: 0 });

                for info in child.infos {
                    self.add_info(info);
                }
                self.emit_event(change::Event::InfosChanged { node: &self });

                return (self.length() == 0, true);
            } else if self.children.as_ref().unwrap().is_empty() {
                self.children = None;
                self.text = Some(String::from(""));
            } else if may_need_regroup {
                self.regroup_neighbors();
            }
        }

        (self.length() == 0, false)
    }

    /// Get a slice of all children under this node.
    pub fn children(&self) -> &[Node<T>] {
        &self.children.as_ref().unwrap()[..]
    }

    /// Give node change listener.
    pub fn give_listener(&mut self, l: &Option<Rc<change::Listener<T>>>) {
        if let Some(v) = l {
            self.listener = Some(Rc::clone(v));
        }
    }

    /// Take the change listener from this node (if any).
    pub fn take_listener(&mut self) -> Option<Rc<change::Listener<T>>> {
        self.listener.take()
    }

    /// Get a depth first pre order iterator.
    pub fn pre_order_iter(&self) -> iterator::PreOrder<T> {
        iterator::PreOrder::new(self)
    }

    /// Get a leaf iterator.
    pub fn leaf_iter(&self) -> impl Iterator<Item=iterator::Item<T>> {
        self.pre_order_iter().filter(|item| item.node.is_leaf())
    }

    /// Emit a change event.
    fn emit_event(&self, event: change::Event<T>) {
        if let Some(l) = &self.listener {
            l(event);
        }
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
