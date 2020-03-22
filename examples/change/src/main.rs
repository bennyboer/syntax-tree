use syntax_tree::Tree;
use shared::info::FontStyle;
use shared::render::html::to_html;
use syntax_tree::change::Event;

fn main() {
    let mut tree: Tree<FontStyle> = Tree::new("Hello World", Some(Box::new(|event| {
        match event {
            Event::NodeAdded { parent, added_idx } => println!(">>> Node with index {} '{}' has been added under {}", added_idx, parent.children()[added_idx].text(), parent.id()),
            Event::NodeRemoved { parent, removed_idx } => println!(">>> Node with index '{}' has been removed under {}", removed_idx, parent.id()),
            Event::InfosChanged { node } => println!(">>> Nodes ({}) infos have been changed to '{:?}'", node.id(), node.infos()),
            Event::TextChanged { node } => println!(">>> Nodes ({}) text has been changed to '{}'", node.id(), node.text()),
        }
    })));

    println!("{:#?}", tree);

    tree.set(6, "Hello World".len(), FontStyle::Bold);
    println!("{:#?}", tree);

    println!("# Could be rendered to HTML like this:");
    println!("{}", to_html(&tree));
}
