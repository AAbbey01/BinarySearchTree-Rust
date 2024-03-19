#![warn(non_snake_case)]
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]

pub mod nodes{
    #[derive(Debug,Clone)]
        ///Creates a node with a value
        /// ````
        /// use unsafe_bst::*;
        /// let n = nodes::Node{val: 14};
        /// ````
        pub struct Node{
            pub val: i64,
        }
        impl Node{
            ///Prints the value of the node
            pub fn print(&self) -> i64{
                print!("{}\n", self.val);
                return self.val;
            }
        }
    }
    ///Module that houses all Binary Tree Operations
    pub mod binary_tree{
        use std::borrow::BorrowMut;
        use crate::{nodes::{self, Node}, stats::Stats};
        pub enum Label{
            Check,
            Else,
            Condition,
        }
    #[derive(Debug,Clone)]
    /**A Binary Tree has a root node, and possibly a left and right subtree.
    Option is used to allow the possibility that a left/right subtree is not defined.
    Box is used to control recursive errors: i.e. there is only a Box when there is a subtree */
    pub struct BinTree{
        ///Holds the Root Node of the tree
        pub root: crate::nodes::Node,
        ///The Possibility of a left subtree
        pub left: Option<Box<BinTree>>, 
        ///The Possibility of a right subtree
        pub right: Option<Box<BinTree>>, 
        
    }
    impl BinTree{
        pub fn find(&self, key: i64) -> bool{
            //!Find is called by a tree and inputs a key. returns true if the key is in the node.
            //! ```rust
            //! use unsafe_bst::*;
            //! let mut tree = binary_tree::BinTree{..Default::default()};
            //! tree.add_node(nodes::Node{val:14});
            //! tree.add_node(nodes::Node{val:15});
            //! tree.add_node(nodes::Node{val:13});
            //! assert_eq!(tree.find(17),false);
            //! assert!(tree.find(14));
            //! ```
            let mut b = false;
            if self.root.val == key{
                //println!("Found {key}");
                b = true;
            }else{
                if self.root.val > key{
                    if self.left.is_some(){       
                        b = self.left.as_ref().unwrap().find(key);
                    }
                }else if self.right.is_some(){
                    b = self.right.as_ref().unwrap().find(key);
                }
            }
            return b;
        }
        pub fn add_node(&mut self, node: crate::nodes::Node){
            //!Add is called by a tree and inputs a Node. The node is added to the tree if it not already in the tree, following basic BST rules
            //! i.e. smaller values of root are to the left, larger values to the right.
            //! ```rust
            //! use unsafe_bst::*;
            //! let mut tree = binary_tree::BinTree{..Default::default()};
            //! tree.add_node(nodes::Node{val:14});
            //! ```
            if node.val == self.root.val {println!("Node already in tree"); return;}
            if node.val == -1 {return;}
            if self.root.val == -1 {
                self.root = node;
                return;
            }
            if self.root.val > node.val{
                match self.left{
                    Some(_) => self.left.as_mut().unwrap().add_node(node),
                    None => self.left = Some(Box::new(BinTree {root: node, ..Default::default() })),
                }         
            }else{
                match self.right{
                    Some(_) => self.right.as_mut().unwrap().add_node(node),
                    None => self.right = Some(Box::new(BinTree {root: node, ..Default::default() })),
                }      
            }
        }
        pub fn print(&self, spacing: i64 ) -> String{
            //!Prints a tree in the following format:
            //! <pre>
            //!     left-subtree
            //! root
            //!     right-subtree
            //! </pre>
            //! spacing should be 0, as 5 spaces are hardcoded into print for each non-root line.
            //! ```rust
            //! use unsafe_bst::*;
            //! let mut tree = binary_tree::BinTree{..Default::default()};
            //! tree.add_node(nodes::Node{val:14});
            //! tree.add_node(nodes::Node{val:15});
            //! tree.add_node(nodes::Node{val:13});
            //! tree.print(0);
            //! ```
            let mut r:String = String::new();
            let space = spacing + 5;
            if self.root.val == -1 {return r;}
            if self.right.is_some() && self.right.as_ref().unwrap().root.val != -1{ 
                let t = self.right.as_ref().unwrap().print(space);
                r = format!("{}{}\n",r,t);
            }
            for _n in 5..=space{
                r = format!("{} ",r);
            }
            r = format!("{}{}\n",r,self.root.val);
            if self.left.is_some() && self.left.as_ref().unwrap().root.val != -1{
                let t = self.left.as_ref().unwrap().print(space);
                r = format!("{}{}\n",r,t);
            }
            return r.trim_end().to_owned();
        }
        pub fn delete( &mut self, node: i64){
            //!Deletes an inputted value from the tree. Follows BST deletion rules:
            //! 1. Leaf node (no left/right-subtree): Node is 'deleted' (set to -1)
            //! 2. Single Child Node (either left or right-subtree): values of tree are shifted up
            //! 3. Two child nodes (left and right sub-tree): successor() is used to find the best replacement for the deleted node
            //! Library Creator Note: This function uses an enum and its types as a way to use GOTO functionality. This is to get around borrowing rules.
            //! ```rust
            //! use unsafe_bst::*;
            //! let mut tree = binary_tree::BinTree{..Default::default()};
            //! tree.add_node(nodes::Node{val:14});
            //! tree.add_node(nodes::Node{val: 3});
            //! tree.delete(14);
            //! //Tree now has root = 3
            //! tree.print(0);
            //! ```
            if !self.find(node){return;}
            let mut label = Label::Check;
            let mut b = self;
            let mut v: Vec<i64> = Vec::new();
            'a: loop{
            match label{
                Label::Check => {
                    if b.root.val == node{
                        label = Label::Condition;
                    }else{
                        label = Label::Else;
                    }
                },
                Label::Else => {
                    if b.root.val > node{
                        if b.left.is_some(){
                            b = b.left.as_mut().unwrap();
                        }
                    }else{
                        if b.right.is_some(){
                            b = b.right.as_mut().unwrap();
                        }
                    }

                    label = Label::Check;
                },
                Label::Condition => {
                        if b.left.is_none() && b.right.is_none() {
                            b.root.val = -1;
                            break 'a;
                        }
                        //get
                        if b.left.is_some() && b.right.is_none() {
                            b.left.as_mut().unwrap().repopulate(&mut v);
                            break 'a;
                        }
                        if b.left.is_none() && b.right.is_some() {
                            b.right.as_mut().unwrap().repopulate(&mut v);
                            break 'a;
                        }
                        if b.left.is_some() && b.right.is_some() {
                            let c = b.successor().root.val;
                            //let temp = b.root.val;
                            b.delete(c);
                            b.root.val = c;
                            break 'a;
                        }
                        
                    },
                }
            } 
        }
        fn repopulate(&mut self, v: &mut Vec<i64>){
            v.push(self.root.val);
            if self.left.is_some(){
                self.left.as_mut().unwrap().repopulate(v);
            }
            if self.right.is_some(){
                self.right.as_mut().unwrap().repopulate(v);
            }
        }
        pub fn successor(&mut self) -> &mut BinTree{
            //!Successor returns a BinTree of the next largest value in the tree from the root (of self)
            //! ```rust
            //! use unsafe_bst::*;
            //! let mut tree = binary_tree::BinTree{..Default::default()};
            //! tree.add_node(nodes::Node{val:14});
            //! tree.add_node(nodes::Node{val:17});
            //! tree.add_node(nodes::Node{val:15});
            //! assert_eq!(15,tree.successor().root.val);
            //! ```
            let mut b: &mut BinTree = self.right.as_mut().unwrap().borrow_mut();
            while b.root.val != -1 {
                if b.left.is_none(){break;}
                else{
                    b = b.shift_left();
                }
            }
            return b;
        }
        pub fn shift_left(&mut self) -> &mut BinTree{
            //!Shift Left returns the left-subtree of a tree
            return self.left.as_mut().unwrap().borrow_mut();
        }
        pub fn shift_right(&mut self) -> &mut BinTree{
            //!Shift Right returns the right-subtree of a tree 
            return unsafe{self.right.as_mut().unwrap_unchecked().borrow_mut()};
        }
        pub fn get_predecessor(&mut self) -> nodes::Node{
            //!Get Predecessor returns the Node with the next smallest value to root (self)
            let mut b: &mut BinTree = self.left.as_mut().unwrap().borrow_mut();
            while b.right.is_some(){
                b = b.shift_right();
            }
            return b.root.clone();
        }
        pub fn get_successor(&mut self) -> nodes::Node{
            //!Get Successor returns the Node with the next largest value to root (self)
            let mut b: &mut BinTree = self.right.as_mut().unwrap().borrow_mut();
            while b.left.is_some(){
                b = b.shift_left();
            }
            return b.root.clone();
        }
        pub fn balance(&mut self, s: &mut Stats) -> BinTree{
            //!Balance attempts to make a perfect BST when it can, else it makes a tree with minimum height
            let mut b_t: BinTree = BinTree{..Default::default()};
            let mut v = s.list.clone();
            v.sort();
            s.list = Vec::new();
            //self is empty, and v has all values
            let a = v.len() as usize;
            b_t.build(&mut v, 0, a-1,  s);
            b_t
        }
        fn build(&mut self, v: &mut Vec<i64>, start: usize, end: usize, s: &mut Stats){
            if start>end{
                return;
            }
            let mid:usize = ((start+end)/2).try_into().unwrap();
            let roo = v[mid];
            s.list.push(roo);
            println!("{roo}");
            self.add_node(Node { val: roo });
            
            if mid>0{
            self.build(v, start, (mid-1).try_into().unwrap(), s);
        }
            self.build(v, (mid+1).try_into().unwrap(), end, s);
            return;
        }
    }
    #[warn(unconditional_recursion)]
    impl Default for BinTree{
        fn default() -> Self {
            BinTree{root: nodes::Node{val: -1}, left: None, right: None}
        }
    }
    }
    ///Module that holds statistics for the BST
    /// #[derive(Debug,Clone)]
    pub mod stats{

        #[derive(Debug,Clone)]
        pub struct Stats{
            pub count: i64,
            pub list: Vec<i64>,
        }
        impl Stats{
            pub fn add(&mut self, val: i64){
                //!Add adds values to a Stats variable if the value is not in it already
                if self.list.iter().position(|&x| x >= val).is_some(){
                    self.list.push(val);
                    self.count +=1;}
            }
            pub fn add_list(&mut self, lis: Vec<i64>) -> bool{
                //!Add_list combines two vectors together, returning true if every value was unique to the stats/tree
                let pre = self.count + lis.len() as i64;
                for l in &lis{
                    self.add(*l);
                    
                }

                if pre != self.list.len() as i64 {
                    return false;
                }
                
                return true;
            }
            pub fn remove(&mut self, val: i64) -> bool{
                //!Removes a value from the list, or returns false
                let  k = 0;
                let b = self.list.remove(self.list.iter().position(|&r | r == val).unwrap_or_else(|| k));
                if b == val{
                    self.count-=1;
                    return true;
                }
                return false;  
            }
            pub fn print_count(&mut self){
                //!Prints # of nodes have been added correctly
                println!("# of Nodes: {}",self.count);
            }
            pub fn print_list(&mut self){
                //!Prints all the node values added correctly
                if self.list.len() == 0{return;}
                print!("List of Nodes: ");
                let last = self.list.last().unwrap();
                for num in &self.list{
                    print!("{}",num);
                    if last == num {print!("\n")} else {print!(", ")}
                }
            }
            pub fn print(&mut self){
                //!Calls print_count() and print_list()
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
pub mod depreciated_functions{
    /* 
    impl crate::binary_tree::BinTree{    
        fn add_next(&mut self, node: crate::nodes::Node){
            if node.val == -1 {return;}
            if self.root.val == -1 {
                self. root = node;
                return;
            }
            if self.root.val > node.val{
                if !self.left.is_some(){
                self.left = Some(Box::new(crate::binary_tree::BinTree{root: node, ..Default::default()}));
                }else{
                unsafe {let _ = &self.left.as_mut().unwrap_unchecked().add_next(node);}
                }    
            }else{
                if !self.right.is_some(){
                    self.right =  Some(Box::new(crate::binary_tree::BinTree{root: node, ..Default::default()}));
                }else{
                    unsafe {let _ = &self.right.as_mut().unwrap_unchecked().add_next(node);}
                }
            }
        }
        pub fn add_2(&mut self, node: nodes::Node){
            if node.val == -1 {return;}
            if self.root.val == -1 {
                self. root = node;
                return;
            }
            if self.root.val > node.val{
                match self.left{
                    Some(_) => {self.left.as_mut().unwrap().add_2(node)},
                    None => self.left = Some(Box::new(BinTree{root: node, ..Default::default()})),
                }
            }else{
               match self.right{
                Some(_) => {self.right.as_mut().unwrap().add_2(node)},
                None => self.right = Some(Box::new(BinTree{root: node, ..Default::default()})),
               }
            }
        }
    
    }
    */
}


#[test]
fn add_node_test(){
    let mut b_t =  binary_tree::BinTree{..Default::default()};
    b_t.add_node(nodes::Node { val: 14 });

    assert_eq!(b_t.root.val, 14);
}
#[test]
fn equal_bst_test(){
   let mut b_t = binary_tree::BinTree{..Default::default()};
   let mut b_t2 = binary_tree::BinTree{..Default::default()};
   
   assert_eq!(b_t.print(0),b_t2.print(0));
   
   b_t.add_node(nodes::Node { val: 14 });
   
   assert_ne!(b_t.print(0),b_t2.print(0));
   
   b_t2.add_node(nodes::Node { val: 14 });
   assert_eq!(b_t.print(0),b_t2.print(0));
}

#[test]
fn print_test(){
    let mut b_t = binary_tree::BinTree{..Default::default()};
    b_t.add_node(nodes::Node { val: 14 });
    b_t.add_node(nodes::Node { val: 13 });
    b_t.add_node(nodes::Node { val: 15 });
    let s = b_t.print(0);
    println!("{s}");
    assert_ne!(s,"");
    
}
#[test]
fn delete_test(){
    let mut b_t = binary_tree::BinTree{..Default::default()};
    let mut b_t2 = binary_tree::BinTree{..Default::default()};
    b_t.add_node(nodes::Node { val: 14 });
    b_t.add_node(nodes::Node { val: 13 });
    b_t.add_node(nodes::Node { val: 15 });
    b_t2.add_node(nodes::Node { val: 14 });
    b_t2.add_node(nodes::Node { val: 13 });
    b_t2.add_node(nodes::Node { val: 15 });
    assert_eq!(b_t.print(0),b_t2.print(0));
    b_t.add_node(nodes::Node { val: 16 });

    println!("b1{}",b_t.print(0));
    println!("b2{}",b_t2.print(0));
    

    assert_ne!(b_t.print(0),b_t2.print(0));
    b_t.delete(16);
    println!("b2{}",b_t.print(0));
    assert_eq!(b_t.print(0),b_t2.print(0));
}