use crate::Node;
use crate::iterator;

/// Depth-first iterator in pre-order.
pub struct PreOrder<'a, T> {
    stack: Vec<iterator::Item<'a, T>>,
}

impl<'a, T> PreOrder<'a, T> {
    pub fn new(root: &'a Node<T>) -> Self {
        PreOrder {
            stack: vec!(iterator::Item {
                node: root,
                level: 0,
            }),
        }
    }
}

impl<'a, T> Iterator for PreOrder<'a, T> {
    type Item = iterator::Item<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            None
        } else {
            let item = self.stack.pop().unwrap();

            if !item.node.is_leaf() {
                let length = item.node.child_count();
                for i in 0..length {
                    self.stack.push(iterator::Item {
                        node: &item.node.children()[length - i - 1],
                        level: item.level + 1,
                    });
                }
            }

            Some(item)
        }
    }
}
