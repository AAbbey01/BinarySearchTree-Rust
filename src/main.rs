#![recursion_limit = "131072"]
//TODO: Add LR actual function to print
use std::io;
//Node Stuff
struct Node{
    val: i64,
}
impl Node{
    
    fn print(&self){
        print!("{}\n", self.val);
    }

}
/**impl Default for Node {
    fn default() -> Node{
    Node {
        val: -1,
    }
    }
    }*/

//Binary Tree Stuff
struct BinTree{
    root: Node,
    left: Option<Box<BinTree>>,
    right: Option<Box<BinTree>>,
    
}
impl BinTree{
   // fn find(&self, key: i64) -> Node{
    //}
    fn add_node(&mut self, node: Node){
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
     fn print(&self){
        
        //unsafe { self.left.as_ref().unwrap_unchecked().print() };
        //unsafe { self.right.as_ref().unwrap_unchecked().print() };
        if self.left.is_some(){
            //if unsafe{&self.left.as_ref().unwrap_unchecked().left}.is_none(){

                unsafe{let _ = &self.left.as_ref().unwrap_unchecked().print();}
            //}
            
        }

        self.root.print();
        if self.right.is_some(){
            //if !unsafe{&self.right.as_ref().unwrap_unchecked().right}.is_none(){
                unsafe{let _ = &self.right.as_ref().unwrap_unchecked().print();}
            //}
            
        }
    }
    /*
    fn find(&self, v: i64){
        if v == self.root.val{
            print!("Value found at root\n");
            return;
        }
        else {
            if v> self.root.val{
                if v == self.right.val{
                    print!("Value found at right\n");
                    return;
                }
            }else{
                if v == self.left.val{
                    print!("Value found at left\n");
                    return;
                }
            }
        }
        print!("Value not found\n");
    } */
    fn add_next(&mut self, node: Node){
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
}
#[warn(unconditional_recursion)]
impl Default for BinTree{
    fn default() -> Self {
        BinTree{root: Node{val: -1}, left: None, right: None}
    }
}

fn main(){
    //n.print();
    let mut b_t = BinTree{..Default::default()};
    //b_t.root.print();
    //b_t.print();
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

    //println!("Numbers entered:");
    for num in numbers {
        //println!("{}", num);
        b_t.add_node(Node{val: num});
    }
    b_t.print();

}