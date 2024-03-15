# unsafe_bst

[![Crates.io](https://img.shields.io/crates/v/unsafe_bst)](https://crates.io/crates/unsafe_bst)
[![Docs.rs](https://docs.rs/unsafe_bst/badge.svg)](https://docs.rs/unsafe_bst/)

# Examples

This will print a tree with a root of 13, right value of 15, and left value of 11
And it will look like this:
    11
13
    15
```rust

let mut b_t = unsafe_bst::binary_tree::BinTree{..Default::default()};
b_t.add_node(unsafe_bst::nodes::Node{val: 13});
b_t.add_node(unsafe_bst::nodes::Node{val: 15});
b_t.add_node(unsafe_bst::nodes::Node{val: 11});
print!("{}",b_t.print(0));
```
