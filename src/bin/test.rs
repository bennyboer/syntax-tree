use syntax_tree::Tree;

fn main() {
    let mut tree: Tree<()> = Tree::new("Hallo Welt");
    tree.set(2, 7, ());

    println!("{:#?}", tree);
}
