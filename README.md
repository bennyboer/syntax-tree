# `syntax-tree`

... is a tree structure managing syntax/format information for text.
It can be used to build the backing model for a WYSIWYG editor or to do syntax highlighting.


## Concepts

- **Only** leafs hold text
- Without syntax/format information the tree consists of only a leaf with all the text
- Every node may contain syntax/format information for **all** underlying nodes
    - Child nodes are not higher prioritized than parent nodes, thus cannot overwrite syntax/format information
- Both nodes and leafs are able to hold syntax/format information
- Only **one** piece of format/syntax information can be held by any node/leaf
- Leafs **must** hold at least one char


## Example

```rust
let mut tree: Tree<()> = Tree::new("Hello World");
tree.set(6, "Hello World".len(), ());
tree.set(0, "Hello World".len(), ());
tree.set(4, 7, ());
tree.insert_str(6, "cold ");
println!("{:#?}", tree);
```

The output should be:

```
|---o ('Hello cold World')
    |---o ('Hello cold World') #
        |-- 'Hell'
        |-- 'o cold ' #
        |---o ('World') #
            |-- 'W' #
            |-- 'orld'
```

The `o` say it is a node with children, the `#` that there is format/syntax information attached to the node/leaf.

For example we could have passed text-format information instead of the `()` in the above code:

- `tree.set(6, "Hello World".len(), ...);` - Format `World` fat
- `tree.set(0, "Hello World".len(), ...);` - Format `Hello World` in italics
- `tree.set(4, 7, ...);` - Format `o W` yellow

After the syntax tree processed the formatting, you can just use the `pre_order_iter` method on the tree to create for example HTML with the formatting information on the nodes in order to create for example formatted HTML text.
