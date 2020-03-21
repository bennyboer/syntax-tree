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
    // let mut tree = Tree::new("Hello World");
    // println!("{:#?}", tree);
    // tree.set(4, 7, Fmt::Underline);
    // println!("{:#?}", tree);
    // tree.set(0, 4, Fmt::Underline);
    // println!("{:#?}", tree);

    let mut tree = Tree::new("Hello World");
    println!("{:#?}", tree);
    tree.set(0, 1, Fmt::Bold);
    println!("{:#?}", tree);
    tree.set(0, 1, Fmt::Italic);
    println!("{:#?}", tree);
    tree.set(0, 1, Fmt::Underline);
    println!("{:#?}", tree);
    tree.set(1, 2, Fmt::Bold);
    println!("{:#?}", tree);
    tree.set(1, 2, Fmt::Italic);
    println!("{:#?}", tree);
    tree.set(2, 3, Fmt::Bold);
    println!("{:#?}", tree);
}
