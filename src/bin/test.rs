use syntax_tree::Tree;
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

fn main() {
    let mut tree = Tree::new("Hello World");
    println!("{:#?}", tree);
    tree.set(6, "Hello World".len(), Fmt::Bold);
    println!("{:#?}", tree);
    tree.set(0, "Hello World".len(), Fmt::Italic);
    println!("{:#?}", tree);
    tree.set(0, "Hello World".len(), Fmt::Bold);
    println!("{:#?}", tree);
    tree.set(4, 7, Fmt::Underline);
    println!("{:#?}", tree);
    tree.insert_str(6, "cold ");
    println!("{:#?}", tree);
}
