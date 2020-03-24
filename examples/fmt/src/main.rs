use syntax_tree::Tree;
use shared::info::FontStyle;
use shared::render::html::to_html;

fn main() {
    println!("# Create new tree with text 'Hello World'");
    let mut tree: Tree<FontStyle> = Tree::new("Hello World", None);
    println!("{:#?}", tree);

    println!("# Format 'o W' underlined");
    tree.set(4, 7, FontStyle::Underline);
    println!("{:#?}", tree);

    println!("# Format 'World' bold");
    tree.set(6, "Hello World".len(), FontStyle::Bold);
    println!("{:#?}", tree);

    println!("# Format 'Wor' underlined");
    tree.set(6, 9, FontStyle::Underline);
    println!("{:#?}", tree);

    println!("# Remove 'o '");
    tree.remove(4, 2);
    println!("{:#?}", tree);

    println!("# Remove format underlined from every node in range 'HellW'");
    tree.unset(0, 5, FontStyle::Underline);
    println!("{:#?}", tree);

    println!("# Format 'ellW' italic");
    tree.set(1, 5, FontStyle::Italic);
    println!("{:#?}", tree);

    println!("# Could be rendered to HTML like this:");
    println!("{}", to_html(&tree));
}
