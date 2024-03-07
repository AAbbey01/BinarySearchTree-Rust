//! Learning Rust: Project for CS471 at SUNY Binghamton
//! ## Detailed Introduction
//! This is a detailed intro to this libraray
//! - This Crate builds a fully operational binary search tree
//! - When run normally, users will be able to interract with the terminal to create and modify the tree
//! - Users will be able to use functions such as:
//!  - [x] adding nodes to a tree
//!  - [ ] deleting nodes
//!  - [x] finding a node in the tree
//!  - [x] displaying the tree
//! Example:
//! ```
//! let mut tree = BinTree{..Default::default()};
//! tree.add_node(Node{val:14});
//! tree.add_node(Node{val:15});
//! tree.add_node(Node{val:13});
//! tree.print();
//! ```
//! 🦀⚠️
#![recursion_limit = "131072"]
#![allow(non_snake_case)]
//TODO: Add LR actual function to print
use std::io;

use BinaryTree::BinTree;
//Node Stuff

pub mod Nodes{
pub struct Node{
    pub(crate) val: i64,
}
impl Node{
    
    pub fn print(&self){
        print!("{}\n", self.val);
    }
}
}
//Binary Tree Stuff
pub mod BinaryTree{
    use crate::Nodes;

pub struct BinTree{
    pub(crate) root: crate::Nodes::Node,
    pub(crate) left: Option<Box<BinTree>>,
    pub(crate) right: Option<Box<BinTree>>,
    
}
impl BinTree{
    
    pub fn find(&self, key: i64) -> bool{
        //print!("{} is the current root\n", self.root.val);
        if self.root.val == key{
            
            return true;
        }else{
            if self.left.is_some(){
                if self.root.val > key{
                unsafe{let _ = &self.left.as_ref().unwrap_unchecked().find(key);
                }
            }}else if self.right.is_some(){
                unsafe{let _ = &self.right.as_ref().unwrap_unchecked().find(key);
                }
            }
        }
        return false;
    }
    pub fn add_node(&mut self, node: crate::Nodes::Node){
        //print!("Adding to {}\n",self.root.val);
        if node.val == -1 {return;}
        if self.root.val == -1 {
            self.root = node;
            return;
        }
        if self.root.val > node.val{
            if !self.left.is_some(){
            self.left = Some(Box::new(BinTree{root: node, ..Default::default()}));
            }else{
               unsafe {let _ = &self.left.as_mut().unwrap_unchecked().add_next(node);}
            }
            
        }else{
            //print!("{0} > {1}\n",node.val,self.root.val);
            if !self.right.is_some(){
                self.right =  Some(Box::new(BinTree{root: node, ..Default::default()}));
            }else{
                unsafe {let _ = &self.right.as_mut().unwrap_unchecked().add_next(node);}
            }
        }
    }
    pub fn print(&self){
        if self.root.val == -1 {return;}
        //unsafe { self.left.as_ref().unwrap_unchecked().print() };
        //unsafe { self.right.as_ref().unwrap_unchecked().print() };
        if self.left.is_some(){
            //if unsafe{&self.left.as_ref().unwrap_unchecked().left}.is_none(){
                print!("    ");
                unsafe{let _ = &self.left.as_ref().unwrap_unchecked().print();}
            //}
            
        }

        self.root.print();
        if self.right.is_some(){
            //if !unsafe{&self.right.as_ref().unwrap_unchecked().right}.is_none(){
                print!("    ");
                unsafe{let _ = &self.right.as_ref().unwrap_unchecked().print();}
            //}
            
        }
    }
    fn add_next(&mut self, node: Nodes::Node){
        if node.val == -1 {return;}
        if self.root.val == -1 {
            self. root = node;
            return;
        }
        if self.root.val > node.val{
            if !self.left.is_some(){
            self.left = Some(Box::new(BinTree{root: node, ..Default::default()}));
            }else{
               unsafe {let _ = &self.left.as_mut().unwrap_unchecked().add_next(node);}
            }
            
        }else{
            if !self.right.is_some(){
                self.right =  Some(Box::new(BinTree{root: node, ..Default::default()}));
            }else{
                unsafe {let _ = &self.right.as_mut().unwrap_unchecked().add_next(node);}
            }
        }
    }
    
    pub fn delete( &mut self, node: i64){
        //Check if the value to be deleted is in the tree in the first place
        if !self.find(node){
            println!("Value not in tree");
            return;};
        //get here if the value is in, then get to the node and check if it has children.
        
    }
    
}
#[warn(unconditional_recursion)]
impl Default for BinTree{
    fn default() -> Self {
        BinTree{root: Nodes::Node{val: -1}, left: None, right: None}
    }
}
}
fn main(){
    //n.print();
    let mut b_t = BinaryTree::BinTree{..Default::default()};
    loop{
        print!("Functions (Type the Letter for Each)\nA: Add a Key to the Tree (Until -1 is inputted)\nF: Find if a Key is in the Tree\nD: Delete A Key in the Tree\nP: Print Tree\nE: Exit (Ctrl+C to Force Stop)\n");
        let mut power_input = String::new();
            io::stdin()
                .read_line(&mut power_input)
                .expect("Failed to read line");
        match power_input.trim().to_uppercase().as_str() {
            "A" =>  add_node_public(&mut b_t),
            "F" => {
                println!("Please enter the key you want to find:");
                let mut ing = String::new();
                    io::stdin()
                        .read_line(&mut ing)
                        .expect("Failed to read line");

                let key: i64 = match ing.trim().parse() {
                    Ok(key) => key,
                    Err(_) => {
                        println!("Invalid input! Please enter an integer.");
                        continue;
                    }
                };
                if b_t.find(key) {
                    print!("{key} found in tree\n");
                }else{
                    print!("{key} not in tree\n");
                }
            },
            "D" => {
                println!("Enter the key you wish to delete: ");
                let mut ing = String::new();
                    io::stdin()
                        .read_line(&mut ing)
                        .expect("Failed to read line");

                let key: i64 = match ing.trim().parse() {
                    Ok(key) => key,
                    Err(_) => {
                        println!("Invalid input! Please enter an integer.");
                        continue;
                    }
                };
                b_t.delete(key);
            },
            "P" => b_t.print(),
            "E" => break,
            _ => print!("Please Input a Valid Arg\n"),
        }
        
        //b_t.root.print();
        //b_t.print();
        
}
}
 
fn add_node_public(n: &mut BinTree){
    let mut numbers = Vec::new();
    println!("Enter integers (enter -1 to exit):");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let num: i64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter an integer.");
                continue;
            }
        };
        if num == -1 {
            break;
        }
        numbers.push(num);
    }
    for num in numbers {
        n.add_node(Nodes::Node{val: num});
    }
}