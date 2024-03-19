# unsafe_bst

[![Crates.io](https://img.shields.io/crates/v/unsafe_bst)](https://crates.io/crates/unsafe_bst)
[![Docs.rs](https://docs.rs/unsafe_bst/badge.svg)](https://docs.rs/unsafe_bst/0.2.2/unsafe_bst/)

# Overview
unsafe_bst's main functionality is a library that can create and maintain a binary search tree. \
The library has declarations for a BST module, a Node module, and a Stat module.
## binary_tree module
The BST module contains a BinTree struct contains 3 fields:
```rust
struct BinTree{
    pub root: crate::nodes::Node,
    pub left: Option<Box<BinTree>>, 
    pub right: Option<Box<BinTree>>,
}
```
The BinTree struct has a default implication:
```rust
let binary_tree = binary_tree::BinTree{..Default::default()};
assert_eq!(binary_tree.root.val, -1);
assert!(binary_tree.left.is_none());
assert!(binary_tree.right.is_none());
```
BinTree's default is a root node of -1, and None for a left and right tree.
## BinTree Functions
BinTree also has default functions built into the library\
 
### add_node(&mut self, node: nodes::Node) 
This function adds a node to the tree, if the node is not already in the tree
### find(&mut self, key: i64) -> bool
This function returns true if the inputted key is in the tree, else returns false
### print(&mut self, spacing: i64)
Prints out the tree in the form
<pre>
   right
root
   left
</pre>
for the entire tree. \
For the best results, spacing should be 0
### delete(&self, key: i64)
Deletes a key from a tree, if the key exists in the tree\
Note: Delete uses GOTO functionality to get through the tree (via enum)
### balance(&self, s: &mut Stats) -> BinTree
Returns <span style="color:red">self</span> as a balanced binary tree

## nodes module
Nodes Module has a struct Node that has the form
```rust
struct Node{
    val: i64,
}
```
## Node functions
Node does not have a default implication but has 1 function
### print(&self)
Prints the value of self.val, the key of the Node

=====================================================

# Examples

This will print a tree with a root of 13, right value of 15, and left value of 11 \
And it will look like this:
<pre>
    11
13
    15
</pre>
```rust
let mut b_t = unsafe_bst::binary_tree::BinTree{..Default::default()};
b_t.add_node(unsafe_bst::nodes::Node{val: 13});
b_t.add_node(unsafe_bst::nodes::Node{val: 15});
b_t.add_node(unsafe_bst::nodes::Node{val: 11});
print!("{}",b_t.print(0));
```

=====================================================


# Changelog:
v0.3.1 - Made delete safer, not 100% safe, added tests to lib.rs\
v0.3.0 - Started to make code safer: removed unsafe calls in every function other than delete\
v0.2.2 - Fixed delete (at least, all seen errors)\
v0.2.1 - fixed balance so that is properly functions\
v0.2.0 - renamed crate to "unsafe-bst"\
v0.1.0 - Created Crate under name "the_one"
