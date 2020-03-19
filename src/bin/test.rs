use syntax_tree::Node;

fn main() {
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

    println!("{:#?}", root);
}
