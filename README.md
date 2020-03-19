# `syntax-tree`

... is a tree structure managing syntax/format information for text.
It can be used to build the backing model for a WYSIWYG editor or to do syntax highlighting.

## Concepts

- **Only** leafs hold text
- Without syntax/format information the tree consists of only a leaf with all the text
- Every node contains syntax/format information for **all** underlying nodes
    - Child nodes are not higher prioritized than parent nodes, thus cannot overwrite syntax/format information
- Leafs **must** hold at least one char
