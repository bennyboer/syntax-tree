use syntax_tree::Tree;
use shared::info::FontStyle;
use shared::render::html::to_html;
use syntax_tree::change::Event;

fn main() {
    let mut tree: Tree<FontStyle> = Tree::new("Hello World");
    tree.add_listener(Box::new(|event| {
        match event {
            Event::NodeAdded { parent, node } => println!("Node {} has been added under {}", node.text(), parent.text()),
            Event::NodeRemoved { parent, removed_idx } => println!("Node with index {} has been removed under {}", removed_idx, parent.text()),
            Event::TextChanged { node } => println!("Nodes text has been changed to {}", node.text()),
        }
    }));

    println!("{:#?}", tree);

    tree.set(6, "Hello World".len(), FontStyle::Bold);
    println!("{:#?}", tree);

    println!("# Could be rendered to HTML like this:");
    println!("{}", to_html(&tree));
}
