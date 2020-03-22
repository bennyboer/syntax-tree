# `syntax-tree` [![Build Status](https://travis-ci.com/bennyboer/syntax-tree.svg?branch=master)](https://travis-ci.com/bennyboer/syntax-tree)

... is a tree structure managing syntax/format information for text.
It can be used to build the backing model for a WYSIWYG editor or to do syntax highlighting.

## Example

The below example code and console output is taken from the example app under `example/fmt/main.rs`.

```rust
println!("# Create new tree with text 'Hello World'");
let mut tree: Tree<Fmt> = Tree::new("Hello World", None);
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
```

The output should be:

```
# Create new tree with text 'Hello World'
|-- 'Hello World' []

# Format 'o W' underlined
|-- 'Hello World' []
    |-- 'Hell' []
    |-- 'o W' [Underline]
    |-- 'orld' []

# Format 'World' bold
|-- 'Hello World' []
    |-- 'Hell' []
    |-- 'o W' [Underline]
        |-- 'o ' []
        |-- 'W' [Bold]
    |-- 'orld' [Bold]

# Format 'Wor' underlined
|-- 'Hello World' []
    |-- 'Hell' []
    |-- 'o W' [Underline]
        |-- 'o ' []
        |-- 'W' [Bold]
    |-- 'orld' [Bold]
        |-- 'or' [Underline]
        |-- 'ld' []

# Remove 'o '
|-- 'HellWorld' []
    |-- 'Hell' []
    |-- 'World' [Bold]
        |-- 'Wor' [Underline]
        |-- 'ld' []

# Remove format underlined from every node in range 'HellW'
|-- 'HellWorld' []
    |-- 'Hell' []
    |-- 'World' [Bold]

# Format 'ellW' italic
|-- 'HellWorld' []
    |-- 'H' []
    |-- 'ell' [Italic]
    |-- 'World' [Bold]
        |-- 'W' [Italic]
        |-- 'orld' []

# Could be rendered to HTML like this:
<p>H<em>ell</em><strong><em>W</em>orld</strong></p>
```

The above example HTML rendering would look like this when rendered in a browser:

<p>H<em>ell</em><strong><em>W</em>orld</strong></p>
