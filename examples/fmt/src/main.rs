use syntax_tree::{Tree, Node};
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
    println!("# Create new tree with text 'Hello World'");
    let mut tree: Tree<Fmt> = Tree::new("Hello World");
    println!("{:#?}", tree);

    println!("# Format 'o W' underlined");
    tree.set(4, 7, Fmt::Underline);
    println!("{:#?}", tree);

    println!("# Format 'World' bold");
    tree.set(6, "Hello World".len(), Fmt::Bold);
    println!("{:#?}", tree);

    println!("# Format 'Wor' underlined");
    tree.set(6, 9, Fmt::Underline);
    println!("{:#?}", tree);

    println!("# Remove 'o '");
    tree.remove(4, 2);
    println!("{:#?}", tree);

    println!("# Remove format underlined from every node in range 'HellW'");
    tree.unset(0, 6, &Fmt::Underline);
    println!("{:#?}", tree);

    println!("# Format 'ellW' italic");
    tree.set(1, 5, Fmt::Italic);
    println!("{:#?}", tree);

    println!("# Could be rendered to HTML like this:");
    println!("{}", to_html(&tree));
}

fn to_html(tree: &Tree<Fmt>) -> String {
    let mut result = String::from("<p>");

    let root = tree.get_root();
    result.push_str(&to_html_node(root));

    result.push_str("</p>");
    result
}

fn to_html_node(node: &Node<Fmt>) -> String {
    let mut result = String::new();

    let mut prefix = String::new();
    let mut postfix = String::new();

    for fmt in node.infos() {
        let tag_name = get_html_tag_name_for_fmt(fmt);
        prefix.push_str(&format!("<{}>", tag_name));
        postfix.push_str(&format!("</{}>", tag_name));
    }

    result.push_str(&prefix);

    if node.is_leaf() {
        result.push_str(&node.text());
    } else {
        for child in node.children() {
            result.push_str(&to_html_node(child));
        }
    }

    result.push_str(&postfix);
    result
}

fn get_html_tag_name_for_fmt(fmt: &Fmt) -> &str {
    match fmt {
        Fmt::Bold => "strong",
        Fmt::Italic => "em",
        Fmt::Underline => "u",
    }
}
