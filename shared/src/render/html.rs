use syntax_tree::{Tree, Node};
use crate::info::FontStyle;

pub fn to_html(tree: &Tree<FontStyle>) -> String {
    let mut result = String::from("<p>");

    let root = tree.get_root();
    result.push_str(&to_html_node(root));

    result.push_str("</p>");
    result
}

fn to_html_node(node: &Node<FontStyle>) -> String {
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

fn get_html_tag_name_for_fmt(fmt: &FontStyle) -> &str {
    match fmt {
        FontStyle::Bold => "strong",
        FontStyle::Italic => "em",
        FontStyle::Underline => "u",
    }
}
