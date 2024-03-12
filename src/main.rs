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

//use BinaryTree::BinTree;
//Node Stuff
const COUNT: i64 = 5;
pub mod Nodes{
#[derive(Debug,Clone)]
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
    use std::borrow::BorrowMut;

    use crate::Nodes;
#[derive(Debug)]
pub struct BinTree{
    pub(crate) root: crate::Nodes::Node,
    pub(crate) left: Option<Box<BinTree>>,
    pub(crate) right: Option<Box<BinTree>>,
    
}
impl BinTree{
    
    pub fn find(&self, key: i64) -> bool{
        //print!("{} is the current root\n", self.root.val);
        let mut b = false;
        if self.root.val == key{
            //println!("Found {key}");
            return true;
        }else{
            if self.root.val > key{
                if self.left.is_some(){   
                    //println!("{key} too small, going left");         
                    unsafe{b = self.left.as_ref().unwrap_unchecked().find(key);}
            }}else if self.right.is_some(){
                //println!("{key} too big, going right");
                unsafe{b = self.right.as_ref().unwrap_unchecked().find(key);}
            }
        }
        return b;
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
    pub fn print(&self, spacing: i64 ){
        let space = spacing + crate::COUNT;
        if self.root.val == -1 {return;}
        if self.left.is_some(){
            unsafe{self.left.as_ref().unwrap_unchecked().print(space);}
        }
        for _n in crate::COUNT..=space{
            print!(" ");
        }
        self.root.print();
        if self.right.is_some(){
            
            unsafe{self.right.as_ref().unwrap_unchecked().print(space);}
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
        if !self.find(node){
            println!("Value not in tree");
            return;
        }
        let mut b: &mut BinTree = self; 
        while b.root.val != node {
           if b.root.val == -1{
            println!("How did you get here, b root is -1");
            return;
           }
           if b.root.val > node{
            b = unsafe{b.left.as_mut().unwrap_unchecked().borrow_mut()};
           }else{
            b = unsafe{b.right.as_mut().unwrap_unchecked().borrow_mut()};
           }
        }
        if b.left.is_none() && b.right.is_none(){b.root.val = -1;}
        //get 
        if b.left.is_some() && b.right.is_none(){b.successLeft();}
        if b.left.is_none() && b.right.is_some(){b.successRight();}
        if b.left.is_some() && b.right.is_some(){
            let c= b.successor().root.val;
            //let temp = b.root.val;
            b.delete(c);
            b.root.val = c;
        }
        
    }
    pub fn successLeft(&mut self){
       // println!("Yes we moved on, to {}",self.root.val);
        if self.left.is_some() {
            self.root.val = unsafe{self.left.as_mut().unwrap_unchecked().root.val};
            unsafe{self.left.as_mut().unwrap_unchecked().successLeft();}
        }else{
            self.root.val = -1;
        }
    }
    pub fn successRight(&mut self){
        //println!("Yes we moved on, to {}",self.root.val);
        if self.right.is_some() {
            self.root.val = unsafe{self.right.as_mut().unwrap_unchecked().root.val};
            unsafe{self.right.as_mut().unwrap_unchecked().successRight();}
        }else{
            self.root.val = -1;
        }
    }
    pub fn successor(&mut self) -> &mut BinTree{
        let mut b: &mut BinTree = unsafe{self.right.as_mut().unwrap_unchecked().borrow_mut()};
        while b.root.val != -1 {
            if b.left.is_none(){break;}
            else{
                b = b.n();
            }

        }
        return b;
    }
    pub fn n(&mut self) -> &mut BinTree{
        return unsafe{self.left.as_mut().unwrap_unchecked().borrow_mut()};
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
        print!("Functions (Type the Letter for Each)\nA: Add a Key to the Tree (Until -1 is inputted)\nF: Find if a Key is in the Tree\nD: Delete A Key in the Tree\nP: Print Tree\nE: Exit (Ctrl+C to Force Stop)\nT: Test Mode\n");
        let mut power_input = String::new();
            io::stdin()
                .read_line(&mut power_input)
                .expect("Failed to read line");
        match power_input.trim().to_uppercase().as_str() {
            "A" =>  {
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
                    b_t.add_node(Nodes::Node{val: num});
                }
            },
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
            "P" => b_t.print(0),
            "E" => break,
            "T" => {
                println!("Test values supplied by my friend Grimgar");
                let numbers = vec![87,1,3,58,99,69,70,31,41,59,26,18];
                for num in numbers {
                    println!("{num} added to the bst");
                    b_t.add_node(Nodes::Node{val: num});
                }
                println!("A print of the tree, before Any Tests");
                b_t.print(0);
                println!("We will remove value 31");
                b_t.delete(31);
                println!("And now Print the tree");
                b_t.print(0);
                println!("Now we will try to delete 90, which is not in the tree");
                b_t.delete(90);
                println!("Now test deleting 58");
                b_t.delete(58);
                b_t.print(0);
                break;
            }
            _ => print!("Please Input a Valid Arg\n"),
        }
        
        //b_t.root.print();
        //b_t.print();
        
}
}
