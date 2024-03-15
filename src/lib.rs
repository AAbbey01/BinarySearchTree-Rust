#![warn(non_snake_case)]
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]


pub mod nodes{
    #[derive(Debug,Clone)]
        ///Creates a node with a value
        /// ````
        /// Node{val: 14};
        /// ````
        pub struct Node{
            pub val: i64,
        }
        impl Node{
            ///Prints the value of the node
            pub fn print(&self){
                print!("{}\n", self.val);
            }
        }
    }
    ///Module that houses all Binary Tree Operations
    pub mod binary_tree{
        use std::borrow::BorrowMut;
        use crate::{nodes::{self}, stats::Stats};
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
        pub fn add_node(&mut self, node: crate::nodes::Node){
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
            let space = spacing + 5;
            if self.root.val == -1 {return;}
            if self.right.is_some(){ 
                unsafe{self.right.as_ref().unwrap_unchecked().print(space);}
            }
            
            for _n in 5..=space{
                print!(" ");
            }
            self.root.print();
            if self.left.is_some(){
                unsafe{self.left.as_ref().unwrap_unchecked().print(space);}
            }
        }
        fn add_next(&mut self, node: nodes::Node){
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
            if b.left.is_some() && b.right.is_none(){b.success_left();}
            if b.left.is_none() && b.right.is_some(){b.success_right();}
            if b.left.is_some() && b.right.is_some(){
                let c= b.successor().root.val;
                //let temp = b.root.val;
                b.delete(c);
                b.root.val = c;
            } 
        }
        pub fn success_left(&mut self){
           // println!("Yes we moved on, to {}",self.root.val);
            if self.left.is_some() {
                self.root.val = unsafe{self.left.as_mut().unwrap_unchecked().root.val};
                unsafe{self.left.as_mut().unwrap_unchecked().success_left();}
            }else{
                self.root.val = -1;
            }
        }
        pub fn success_right(&mut self){
            //println!("Yes we moved on, to {}",self.root.val);
            if self.right.is_some() {
                self.root.val = unsafe{self.right.as_mut().unwrap_unchecked().root.val};
                unsafe{self.right.as_mut().unwrap_unchecked().success_right();}
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
        pub fn get_predecessor(&mut self) -> nodes::Node{
            let mut b: &mut BinTree = unsafe{self.left.as_mut().unwrap_unchecked().borrow_mut()};
            while b.right.is_some(){
                b = b.shift_right();
            }
            return b.root.clone();
        }
        pub fn get_successor(&mut self) -> nodes::Node{
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
                b_b.add_node(nodes::Node { val: (num) });
            } 
            b_b.print(0);
        }
        pub fn add_2(&mut self, node: nodes::Node){
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