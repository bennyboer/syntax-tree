use syntax_tree::Tree;

fn main() {
    let mut tree: Tree<()> = Tree::new("Hello World");
    tree.set(6, "Hello World".len(), ());
    tree.set(0, "Hello World".len(), ());
    tree.set(4, 7, ());
    tree.insert_str(6, "cold ");
    println!("{:#?}", tree);
}
