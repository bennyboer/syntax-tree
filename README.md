# `syntax-tree` [![Build Status](https://travis-ci.com/bennyboer/syntax-tree.svg?branch=master)](https://travis-ci.com/bennyboer/syntax-tree)

... is a tree structure managing syntax/format information for text.
It can be used to build the backing model for a WYSIWYG editor or to do syntax highlighting.

## Example

The below example code and console output is taken from the example app under `example/fmt/main.rs`.

```rust
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
        |-- 'W' []
        |-- 'or' [Underline]
        |-- 'ld' []

# Format 'ellW' italic
|-- 'HellWorld' []
    |-- 'H' []
    |-- 'ell' [Italic]
    |-- 'World' [Bold]
        |-- 'W' [Italic]
        |-- 'or' [Underline]
        |-- 'ld' []

# Could be rendered to HTML like this:
<p>H<em>ell</em><strong><em>W</em><u>or</u>ld</strong></p>
```

The above example HTML rendering would look like this when rendered in a browser:

<p>H<em>ell</em><strong><em>W</em><u>or</u>ld</strong></p>
