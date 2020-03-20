use std::fmt;
use crate::{Node, iterator};
use std::rc::Rc;
use std::hash::Hash;
use std::fmt::Debug;

pub struct Tree<T> {
    /// The trees root node.
    root: Node<T>,
}

impl<T> Tree<T>
    where T: Eq + Hash {
    /// Create new tree.
    pub fn new(string: &str) -> Tree<T> {
        Tree {
            root: Node::new_root(string),
        }
    }

    /// Load a tree.
    pub fn load() -> Tree<T> {
        // TODO Implement loading a tree from syntax/format information and text
        unimplemented!()
    }

    /// Set syntax/format info for the passed range.
    /// The range is the passed start index (inclusive) to the passed end index (exclusive).
    pub fn set(&mut self, start_idx: usize, end_idx: usize, obj: T) {
        self.root.set(start_idx, end_idx, Rc::new(obj));
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

#[cfg(test)]
mod tests {
    use crate::Tree;
    use std::cmp::Ordering;

    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
    pub enum Fmt {
        Bold = 1,
        Italic = 2,
        Underline = 3,
    }

    impl Ord for Fmt {
        fn cmp(&self, other: &Self) -> Ordering {
            let a = *self as u8;
            let b = *other as u8;

            a.cmp(&b)
        }
    }

    #[test]
    #[should_panic]
    fn format_test_leaf_split_invalid_input_3() {
        let mut tree = Tree::new("Hallo Welt");
        tree.set(2, 1, Fmt::Bold);
    }

    #[test]
    fn format_test_leaf_split_case_1() {
        let mut tree = Tree::new("Hallo Welt");
        tree.set(0, "Hallo Welt".len(), Fmt::Bold);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hallo Welt' [Bold]
");
    }

    #[test]
    fn format_test_leaf_split_case_2() {
        let mut tree = Tree::new("Hallo Welt");
        tree.set(0, 5, Fmt::Bold);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hallo Welt' []
    |-- 'Hallo' [Bold]
    |-- ' Welt' []
");
    }

    #[test]
    fn format_test_leaf_split_case_3() {
        let mut tree = Tree::new("Hallo Welt");
        tree.set(6, "Hallo Welt".len(), Fmt::Bold);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hallo Welt' []
    |-- 'Hallo ' []
    |-- 'Welt' [Bold]
");
    }

    #[test]
    fn format_test_leaf_split_case_4() {
        let mut tree = Tree::new("Hallo Welt");
        tree.set(2, 7, Fmt::Bold);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hallo Welt' []
    |-- 'Ha' []
    |-- 'llo W' [Bold]
    |-- 'elt' []
");
    }

    #[test]
    fn format_test_complex_1() {
        let mut tree = Tree::new("Hallo Welt");
        tree.set(6, "Hallo Welt".len(), Fmt::Bold);
        tree.set(4, 7, Fmt::Underline);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hallo Welt' []
    |-- 'Hall' []
    |-- 'o ' [Underline]
    |-- 'Welt' [Bold]
        |-- 'W' [Underline]
        |-- 'elt' []
");
    }

    #[test]
    fn format_test_complex_2() {
        let mut tree = Tree::new("Hallo Welt");
        tree.set(6, "Hallo Welt".len(), Fmt::Bold);
        tree.set(0, "Hallo Welt".len(), Fmt::Italic);
        tree.set(4, 7, Fmt::Underline);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hallo Welt' [Italic]
    |-- 'Hall' []
    |-- 'o ' [Underline]
    |-- 'Welt' [Bold]
        |-- 'W' [Underline]
        |-- 'elt' []
");
    }

    #[test]
    fn insert_str_test_complex() {
        let mut tree = Tree::new("Hallo Welt");
        tree.set(6, "Hallo Welt".len(), Fmt::Bold);
        tree.set(0, "Hallo Welt".len(), Fmt::Italic);
        tree.set(4, 7, Fmt::Underline);
        tree.insert_str(6, "du ");

        assert_eq!(format!("{:#?}", tree), "|-- 'Hallo du Welt' [Italic]
    |-- 'Hall' []
    |-- 'o du ' [Underline]
    |-- 'Welt' [Bold]
        |-- 'W' [Underline]
        |-- 'elt' []
");
    }

    #[test]
    fn format_test_complex_3() {
        let mut tree: Tree<i32> = Tree::new("Hello World");
        tree.set(6, "Hello World".len(), 3);
        tree.set(0, "Hello World".len(), 42);
        tree.set(0, "Hello World".len(), 8);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hello World' [8, 42]
    |-- 'Hello ' []
    |-- 'World' [3]
")
    }

    #[test]
    fn format_test_cleanup_1() {
        let mut tree = Tree::new("Hello World");
        tree.set(6, "Hello World".len(), Fmt::Bold);
        tree.set(0, "Hello World".len(), Fmt::Italic);
        tree.set(0, "Hello World".len(), Fmt::Bold);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hello World' [Bold, Italic]
")
    }

    #[test]
    fn format_test_cleanup_2() {
        let mut tree = Tree::new("Hello World");
        tree.set(6, "Hello World".len(), Fmt::Bold);
        tree.set(0, 6, Fmt::Underline);
        tree.set(0, "Hello World".len(), Fmt::Italic);
        tree.set(0, "Hello World".len(), Fmt::Bold);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hello World' [Bold, Italic]
    |-- 'Hello ' [Underline]
    |-- 'World' []
")
    }

    #[test]
    fn format_test_cleanup_3() {
        let mut tree = Tree::new("Hello World");
        tree.set(6, "Hello World".len(), Fmt::Bold);
        tree.set(0, 6, Fmt::Underline);
        tree.set(0, "Hello World".len(), Fmt::Italic);
        tree.set(0, "Hello World".len(), Fmt::Bold);
        tree.set(0, "Hello World".len(), Fmt::Underline);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hello World' [Bold, Italic, Underline]
")
    }
}
