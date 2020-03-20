use syntax_tree::Tree;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Fmt {
    Bold,
    Italic,
    Underline,
}

fn main() {
    // let mut tree = Tree::new("Hello World");
    // println!("{:#?}", tree);
    // tree.set(6, "Hello World".len(), Fmt::Bold);
    // println!("{:#?}", tree);
    // tree.set(0, "Hello World".len(), Fmt::Italic);
    // println!("{:#?}", tree);
    // tree.set(4, 7, Fmt::Underline);
    // println!("{:#?}", tree);
    // tree.insert_str(6, "cold ");

    let mut tree: Tree<()> = Tree::new("Hallo Welt");
    tree.set(0, "Hallo Welt".len(), ());

    println!("{:#?}", tree);
}
