//! Learning Rust: Project for CS471 at SUNY Binghamton
//! ## Detailed Introduction
//! This is a detailed intro to this libraray
//! - This Crate builds a fully operational binary search tree
//! - When run normally, users will be able to interract with the terminal to create and modify the tree
//! - Users will be able to use functions such as:
//!  - [x] adding nodes to a tree
//!  - [x] deleting nodes
//!  - [x] finding a node in the tree
//!  - [x] displaying the tree
//! Example:
//! ```rust
//! let mut tree = BinTree{..Default::default()};
//! tree.add_node(Node{val:14});
//! tree.add_node(Node{val:15});
//! tree.add_node(Node{val:13});
//! tree.print(0);
//! tree.delete(14);
//! tree.print(0);
//! ```
//! 🦀⚠️
#![recursion_limit = "131072"]
#![allow(non_snake_case)]
use std::{fs::File, io::{self, BufRead, BufReader}};
static PATH:&str = "src\\user_params.txt";
//use eframe::egui;
///This is a constant variable for spacing when printing the Tree.
const COUNT: i64 = 5;
///Simple module defining the Node structure and its implementations
pub mod Nodes{
#[derive(Debug,Clone)]
    ///Creates a node with a value
    /// ````
    /// Node{val: 14};
    /// ````
    pub struct Node{
        pub(crate) val: i64,
    }
    impl Node{
        ///Prints the value of the node
        pub fn print(&self){
            print!("{}\n", self.val);
        }
    }
}
///Module that houses all Binary Tree Operations
pub mod BinaryTree{
    use std::borrow::BorrowMut;
    use crate::{Nodes::{self}, Stats::Stats};
#[derive(Debug,Clone)]
/**A Binary Tree has a root node, and possibly a left and right subtree.
Option is used to allow the possibility that a left/right subtree is not defined.
Box is used to control recursive errors: i.e. there is only a Box when there is a subtree */
pub struct BinTree{
    ///Holds the Root Node of the tree
    pub(crate) root: crate::Nodes::Node,
    ///The Possibility of a left subtree
    pub(crate) left: Option<Box<BinTree>>, 
    ///The Possibility of a right subtree
    pub(crate) right: Option<Box<BinTree>>, 
    
}
impl BinTree{
    pub fn find(&self, key: i64) -> bool{
        //!Find is called by a tree and inputs a key. returns true if the key is in the node.
        //! ```rust
        //! let mut tree = BinTree{..Default::default()};
        //! tree.add_node(Node{val:14});
        //! tree.add_node(Node{val:15});
        //! tree.add_node(Node{val:13});
        //! asserteq!(tree.find(17,false));
        //! assert!(tree.find(14));
        //! ```
        let mut b = false;
        if self.root.val == key{
            //println!("Found {key}");
            b = true;
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
        if node.val == self.root.val {println!("Node already in tree"); return;}
        if node.val == -1 {return;}
        if self.root.val == -1 {
            self.root = node;
            return;
        }
        if self.root.val > node.val{
            if !self.left.is_some(){
            self.left = Some(Box::new(BinTree{root: node, ..Default::default()}));
            }else{
               unsafe {let _ = &self.left.as_mut().unwrap_unchecked().add_node(node);}
            }
            
        }else{
            //print!("{0} > {1}\n",node.val,self.root.val);
            if !self.right.is_some(){
                self.right =  Some(Box::new(BinTree{root: node, ..Default::default()}));
            }else{
                unsafe {let _ = &self.right.as_mut().unwrap_unchecked().add_node(node);}
            }
        }
    }
    pub fn print(&self, spacing: i64 ){
        let space = spacing + crate::COUNT;
        if self.root.val == -1 {return;}
        if self.right.is_some(){ 
            unsafe{self.right.as_ref().unwrap_unchecked().print(space);}
        }
        
        for _n in crate::COUNT..=space{
            print!(" ");
        }
        self.root.print();
        if self.left.is_some(){
            unsafe{self.left.as_ref().unwrap_unchecked().print(space);}
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
                b = b.shift_left();
            }
        }
        return b;
    }
    pub fn shift_left(&mut self) -> &mut BinTree{
        return unsafe{self.left.as_mut().unwrap_unchecked().borrow_mut()};
    }
    pub fn shift_right(&mut self) -> &mut BinTree{
        return unsafe{self.right.as_mut().unwrap_unchecked().borrow_mut()};
    }
    pub fn get_predecessor(&mut self) -> Nodes::Node{
        let mut b: &mut BinTree = unsafe{self.left.as_mut().unwrap_unchecked().borrow_mut()};
        while b.right.is_some(){
            b = b.shift_right();
        }
        return b.root.clone();
    }
    pub fn get_successor(&mut self) -> Nodes::Node{
        let mut b: &mut BinTree = unsafe{self.right.as_mut().unwrap_unchecked().borrow_mut()};
        while b.left.is_some(){
            b = b.shift_left();
        }
        return b.root.clone();
    }
    pub fn balance(&mut self, s: Stats){
        let mut b_b = BinTree{..Default::default()};
        let mut t:Vec<i64> = Vec::new();
        for num in s.list{
            t.push(num);
        }
        t.sort();
        for num in t{
            b_b.add_node(Nodes::Node { val: (num) });
        } 
        b_b.print(0);
    }
    pub fn add_2(&mut self, node: Nodes::Node){
        if node.val == -1 {return;}
        if self.root.val == -1 {
            self. root = node;
            return;
        }
        if self.root.val > node.val{
            match self.left{
                Some(_) => {self.left.as_mut().unwrap().add_next(node)},
                None => self.left = Some(Box::new(BinTree{root: node, ..Default::default()})),
            }
        }else{
           match self.right{
            Some(_) => {self.right.as_mut().unwrap().add_next(node)},
            None => self.right = Some(Box::new(BinTree{root: node, ..Default::default()})),
           }
        }
    }
}
#[warn(unconditional_recursion)]
impl Default for BinTree{
    fn default() -> Self {
        BinTree{root: Nodes::Node{val: -1}, left: None, right: None}
    }
}
}
///Module that holds statistics for the BST
/// #[derive(Debug,Clone)]
pub mod Stats{
    #[derive(Debug,Clone)]
    pub struct Stats{
        pub(crate)count: i64,
        pub(crate)list: Vec<i64>,
    }
    impl Stats{
        pub fn add(&mut self, val: i64){
            self.list.push(val);
                self.count +=1;
        }
        pub fn add_list(&mut self, mut lis: Vec<i64>) -> bool{

            self.list.append(&mut lis);
            if self.list.len() as i64 != self.count + lis.len() as i64 {
                return false;
            }
            self.count += lis.len() as i64;
            return true;
        }
        pub fn remove(&mut self, val: i64) -> bool{
            let  k = 0;
            let b = self.list.remove(self.list.iter().position(|&r | r == val).unwrap_or_else(|| k));
            //self.list.remove(self.list.iter().position(|&r | r == val).unwrap_or_else(b = false)); 
            if b == val{
                self.count-=1;
                return true;
            }
            return false;  
        }
        pub fn print_count(&mut self){
            println!("# of Nodes: {}",self.count);
        }
        pub fn print_list(&mut self){
            print!("List of Nodes: ");
            let last = self.list.last().unwrap();
            for num in &self.list{
                print!("{}",num);
                if last == num {print!("\n")} else {print!(", ")}
            }
        }
        pub fn print(&mut self){
            self.print_count();
            self.print_list();
        }
    }

    impl Default for Stats{
        fn default() -> Self {
            Stats{count: 0, list: Vec::new()}
        }
    }

}

fn main(){
    let mut b_t = BinaryTree::BinTree{..Default::default()};
    let mut stat_track = Stats::Stats{..Default::default()};    
    let file:File = File::open(PATH).expect("Reason");
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().collect();
    
    loop{
        for l in &lines{
            match l{
                Ok(t) => println!("{}",t),
                _ => {},
            }
        }
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
                    stat_track.add(num);
                }
            },
            "B" =>{
                let mut bal = b_t.clone();
                let s = stat_track.clone();
                bal.balance(s);
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
            "T1" => {
                test_1(b_t);
                break;
            }
            "T2" =>{test_2(b_t); return;},
            "T3" => {test_3(b_t); return;},
            "GP" => {print!("Root nodes predecessor: "); b_t.get_predecessor().print();},
            "GS" => {print!("Root nodes successor: "); b_t.get_successor().print();},
            "S" => {
                stat_track.print_count();
                stat_track.print_list();

            },
            "R" => {
                println!("Enter how many nodes you want:  ");
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
                let mut rng = rand::thread_rng();
                for _num in 1..key{
                    let x:i64 = rng.gen();
                    b_t.add_node(Nodes::Node { val: (x) });
                    stat_track.add(x);
                    stat_track.list.sort();
                }
            }
            _ => print!("Please Input a Valid Arg\n"),
        } //match input statement
        
        
    }//inf loop for input
}
/*fn pause() {

    print!("Press any key to continue");
        let mut _pause = String::new();
            io::stdin()
                .read_line( &mut pause)
                .expect("Failed to read line");
}*/
fn test_1(mut b_t: BinaryTree::BinTree){
    let mut stat_t1 = Stats::Stats{..Default::default()};
    println!("Test values supplied by my friend Grimgar");
    let numbers = vec![87,1,3,58,99,69,70,31,41,59,26,18];
    for num in numbers {
        println!("{num} added to the bst");
        b_t.add_2(Nodes::Node{val: num});
        stat_t1.add(num);
    }
    println!("A print of the tree, before Any Tests");
    b_t.print(0);
    stat_t1.print();
    println!("We will remove value 31");
    b_t.delete(31);
    stat_t1.remove(31);
    println!("And now Print the tree");
    b_t.print(0);
    stat_t1.print();
    println!("Now we will try to delete 90, which is not in the tree");
    b_t.delete(90);
    stat_t1.remove(90);
    println!("Now test deleting 58");
    b_t.delete(58);
    b_t.print(0);
    stat_t1.remove(58);
    stat_t1.print();
}
use rand::prelude::*;

fn test_2(mut b_t: BinaryTree::BinTree){
    let mut stats_test2 = Stats::Stats{..Default::default()};
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i64> = (0..32).collect();
    nums.shuffle(&mut rng);
    for n in nums{
        b_t.add_node(Nodes::Node { val: (n) });
        stats_test2.add(n);
    }
    b_t.print(0);
    println!("Root Node: {}", b_t.root.val);
    println!("Root Predecessor: {}",b_t.get_predecessor().val);
    println!("Root Successor: {}", b_t.get_successor().val);
    stats_test2.print_count();
    stats_test2.print_list();
    println!("Tree\\{}",b_t.root.val);
    stats_test2.remove(b_t.root.val);
    b_t.delete(b_t.root.val);
    b_t.print(0);
    stats_test2.print_count();
    stats_test2.print_list();
}
fn test_3(mut b_t: BinaryTree::BinTree){
    let mut s: Stats::Stats = Stats::Stats { ..Default::default() };
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i64> = (1..32).collect();
    nums.shuffle(&mut rng);
    for n in nums{
        b_t.add_node(Nodes::Node { val: (n) });
        s.add(n);
    }
    b_t.print(0);
    let mut bal = b_t.clone();
    let st = s.clone();
    bal.balance(st);
}
