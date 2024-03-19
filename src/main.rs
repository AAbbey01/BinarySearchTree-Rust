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
///This is a constant variable for spacing when printing the Tree.
///Simple module defining the Node structure and its implementations
fn main(){
    let mut b_t = unsafe_bst::binary_tree::BinTree{..Default::default()};
    let mut stat_track = unsafe_bst::stats::Stats{..Default::default()};    
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
                    b_t.add_node(unsafe_bst::nodes::Node{val: num});
                    stat_track.add(num);
                }
            },
            "B" =>{
                b_t = b_t.balance(&mut stat_track);
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
                //stat_track.remove(key);
            },
            "P" => println!("{}",b_t.print_2(0,1)),
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
                    b_t.add_node(unsafe_bst::nodes::Node { val: (x) });
                    stat_track.add(x);
                    stat_track.list.sort();
                }
            }
            _ => print!("Please Input a Valid Arg\n"),
        } //match input statement
    }//inf loop for input
}
fn test_1(mut b_t: unsafe_bst::binary_tree::BinTree){
    let mut stat_t1 = unsafe_bst::stats::Stats{..Default::default()};
    println!("Test values supplied by my friend Grimgar");
    let numbers = vec![87,1,3,58,99,69,70,31,41,59,26,18];
    for num in numbers {
        println!("{num} added to the bst");
        b_t.add_node(unsafe_bst::nodes::Node{val: num});
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

fn test_2(mut b_t: unsafe_bst::binary_tree::BinTree){
    let mut stats_test2 = unsafe_bst::stats::Stats{..Default::default()};
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i64> = (0..32).collect();
    nums.shuffle(&mut rng);
    for n in nums{
        b_t.add_node(unsafe_bst::nodes::Node { val: (n) });
        stats_test2.add(n);
    }
    println!("{}",b_t.print(0));
    println!("Root Node: {}", b_t.root.val);
    println!("Root Predecessor: {}",b_t.get_predecessor().val);
    println!("Root Successor: {}", b_t.get_successor().val);
    stats_test2.print_count();
    stats_test2.print_list();
    println!("Tree\\{}",b_t.root.val);
    //stats_test2.remove(b_t.root.val);
    b_t.delete(b_t.root.val);
    println!("{}",b_t.print(0));
    stats_test2.print_count();
    stats_test2.print_list();
}
fn test_3(mut b_t: unsafe_bst::binary_tree::BinTree){
    let mut s: unsafe_bst::stats::Stats = unsafe_bst::stats::Stats { ..Default::default() };
    let mut rng = rand::thread_rng();
    let mut nums: Vec<i64> = (1..32).collect();
    nums.shuffle(&mut rng);
    for n in nums{
        b_t.add_node(unsafe_bst::nodes::Node { val: (n) });
        s.add(n);
    }
    println!("{}",b_t.print_2(0,1));
    b_t = b_t.balance(&mut s);
    println!("Perfectly Balanced Tree");
    println!("{}",b_t.print_2(0,1));
}