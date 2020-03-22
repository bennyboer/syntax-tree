#[cfg(test)]
mod tests {
    use syntax_tree::{Tree, Node};
    use shared::info::FontStyle;

    #[test]
    #[should_panic]
    fn format_test_leaf_split_invalid_input_3() {
        let mut tree = Tree::new("Hallo Welt");
        tree.set(2, 1, FontStyle::Bold);
    }

    #[test]
    fn format_test_leaf_split_case_1() {
        let mut tree = Tree::new("Hallo Welt");
        tree.set(0, "Hallo Welt".len(), FontStyle::Bold);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hallo Welt' [Bold]
");
    }

    #[test]
    fn format_test_leaf_split_case_2() {
        let mut tree = Tree::new("Hallo Welt");
        tree.set(0, 5, FontStyle::Bold);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hallo Welt' []
    |-- 'Hallo' [Bold]
    |-- ' Welt' []
");
    }

    #[test]
    fn format_test_leaf_split_case_3() {
        let mut tree = Tree::new("Hallo Welt");
        tree.set(6, "Hallo Welt".len(), FontStyle::Bold);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hallo Welt' []
    |-- 'Hallo ' []
    |-- 'Welt' [Bold]
");
    }

    #[test]
    fn format_test_leaf_split_case_4() {
        let mut tree = Tree::new("Hallo Welt");
        tree.set(2, 7, FontStyle::Bold);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hallo Welt' []
    |-- 'Ha' []
    |-- 'llo W' [Bold]
    |-- 'elt' []
");
    }

    #[test]
    fn format_test_complex_1() {
        let mut tree = Tree::new("Hallo Welt");
        tree.set(6, "Hallo Welt".len(), FontStyle::Bold);
        tree.set(4, 7, FontStyle::Underline);

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
        tree.set(6, "Hallo Welt".len(), FontStyle::Bold);
        tree.set(0, "Hallo Welt".len(), FontStyle::Italic);
        tree.set(4, 7, FontStyle::Underline);

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
        tree.set(6, "Hallo Welt".len(), FontStyle::Bold);
        tree.set(0, "Hallo Welt".len(), FontStyle::Italic);
        tree.set(4, 7, FontStyle::Underline);
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
");
    }

    #[test]
    fn format_test_cleanup_1() {
        let mut tree = Tree::new("Hello World");
        tree.set(6, "Hello World".len(), FontStyle::Bold);
        tree.set(0, "Hello World".len(), FontStyle::Italic);
        tree.set(0, "Hello World".len(), FontStyle::Bold);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hello World' [Bold, Italic]
");
    }

    #[test]
    fn format_test_cleanup_2() {
        let mut tree = Tree::new("Hello World");
        tree.set(6, "Hello World".len(), FontStyle::Bold);
        tree.set(0, 6, FontStyle::Underline);
        tree.set(0, "Hello World".len(), FontStyle::Italic);
        tree.set(0, "Hello World".len(), FontStyle::Bold);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hello World' [Bold, Italic]
    |-- 'Hello ' [Underline]
    |-- 'World' []
");
    }

    #[test]
    fn format_test_cleanup_3() {
        let mut tree = Tree::new("Hello World");
        tree.set(6, "Hello World".len(), FontStyle::Bold);
        tree.set(0, 6, FontStyle::Underline);
        tree.set(0, "Hello World".len(), FontStyle::Italic);
        tree.set(0, "Hello World".len(), FontStyle::Bold);
        tree.set(0, "Hello World".len(), FontStyle::Underline);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hello World' [Bold, Italic, Underline]
");
    }

    #[test]
    fn unset_test_1() {
        let mut tree = Tree::new("Hello World");
        tree.set(4, 7, FontStyle::Underline);
        tree.unset(6, 7, &FontStyle::Underline);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hello World' []
");
    }

    #[test]
    fn group_neighbor_test_1() {
        let mut tree = Tree::new("Hello World");
        tree.set(4, 7, FontStyle::Underline);
        tree.set(4, 7, FontStyle::Bold);
        tree.set(0, 4, FontStyle::Underline);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hello World' []
    |-- 'Hello W' [Underline]
        |-- 'Hell' []
        |-- 'o W' [Bold]
    |-- 'orld' []
");
    }

    #[test]
    fn group_neighbor_test_2() {
        let mut tree = Tree::new("Hello World");
        tree.set(0, 1, FontStyle::Bold);
        tree.set(0, 1, FontStyle::Italic);
        tree.set(0, 1, FontStyle::Underline);
        tree.set(1, 2, FontStyle::Bold);
        tree.set(1, 2, FontStyle::Italic);
        tree.set(2, 3, FontStyle::Bold);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hello World' []
    |-- 'Hel' [Bold]
        |-- 'He' [Italic]
            |-- 'H' [Underline]
            |-- 'e' []
        |-- 'l' []
    |-- 'lo World' []
");
    }

    #[test]
    fn clear_test() {
        let mut tree = Tree::new("Hello World");
        tree.set(6, "Hello World".len(), FontStyle::Bold);
        tree.set(0, 6, FontStyle::Underline);
        tree.set(0, "Hello World".len(), FontStyle::Italic);
        tree.set(0, "Hello World".len(), FontStyle::Bold);
        tree.set(0, "Hello World".len(), FontStyle::Underline);

        tree.clear(true);

        assert_eq!(format!("{:#?}", tree), "|-- '' [Bold, Italic, Underline]
");

        tree.push_str("Hello World");
        tree.set(6, "Hello World".len(), FontStyle::Bold);
        tree.set(0, 6, FontStyle::Underline);
        tree.set(0, "Hello World".len(), FontStyle::Italic);
        tree.set(0, "Hello World".len(), FontStyle::Bold);
        tree.set(0, "Hello World".len(), FontStyle::Underline);

        tree.clear(false);

        assert_eq!(format!("{:#?}", tree), "|-- '' []
");
    }

    #[test]
    fn remove_test_1() {
        let mut tree: Tree<FontStyle> = Tree::new("Hello World");
        tree.pop();
        tree.remove(3, 4);

        assert_eq!(format!("{:#?}", tree), "|-- 'Helorl' []
");
    }

    #[test]
    fn remove_test_2() {
        let mut tree: Tree<FontStyle> = Tree::new("Hello World");
        tree.set(4, 7, FontStyle::Underline);
        tree.set(4, "Hello World".len(), FontStyle::Bold);
        tree.set(0, 4, FontStyle::Underline);
        tree.pop();
        tree.remove(3, tree.length() - 3);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hel' [Underline]
")
    }

    #[test]
    fn remove_test_3() {
        let mut tree: Tree<FontStyle> = Tree::new("Hello World");
        tree.set(4, 7, FontStyle::Underline);
        tree.set(4, "Hello World".len(), FontStyle::Bold);
        tree.set(0, 4, FontStyle::Underline);
        tree.pop();
        tree.remove(3, 5);

        assert_eq!(format!("{:#?}", tree), "|-- 'Helrl' []
    |-- 'Hel' [Underline]
    |-- 'rl' [Bold]
")
    }

    #[test]
    fn remove_test_regroup_1() {
        let mut tree: Tree<FontStyle> = Tree::new("Hello World");
        tree.set(4, 7, FontStyle::Underline);
        tree.set(4, "Hello World".len(), FontStyle::Bold);
        tree.set(0, 4, FontStyle::Underline);
        tree.remove(7, "Hello World".len());

        assert_eq!(format!("{:#?}", tree), "|-- 'Hello W' []
    |-- 'Hello W' [Underline]
        |-- 'Hell' []
        |-- 'o W' [Bold]
")
    }

    #[test]
    fn remove_test_regroup_2() {
        let mut tree: Tree<FontStyle> = Tree::new("Hello World");
        tree.set(4, 7, FontStyle::Underline);
        tree.set(6, "Hello World".len(), FontStyle::Bold);
        tree.set(6, 9, FontStyle::Underline);
        tree.remove(4, 2);

        assert_eq!(format!("{:#?}", tree), "|-- 'HellWorld' []
    |-- 'Hell' []
    |-- 'World' [Bold]
        |-- 'Wor' [Underline]
        |-- 'ld' []
")
    }

    #[test]
    fn set_test_info_already_set_on_node() {
        let mut tree: Tree<FontStyle> = Tree::new("Hello World");
        tree.set(4, 7, FontStyle::Underline);
        tree.set(6, "Hello World".len(), FontStyle::Bold);
        tree.set(6, 9, FontStyle::Underline);

        assert_eq!(format!("{:#?}", tree), "|-- 'Hello World' []
    |-- 'Hell' []
    |-- 'o W' [Underline]
        |-- 'o ' []
        |-- 'W' [Bold]
    |-- 'orld' [Bold]
        |-- 'or' [Underline]
        |-- 'ld' []
")
    }

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
