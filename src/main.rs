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
    
    fn find(&self, key: i64){
        print!("{} is the current root\n", self.root.val);
        if self.root.val == key{
            print!("Found {key} in tree");
            return;
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
        print!("Didn't Find {key} in tree");
    }
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
    loop{
        print!("Functions (Type the Letter for Each)\nA: Add a Key to the Tree (Until -1 is inputted)\nF: Find if a Key is in the Tree\nD: Delete A Key in the Tree\nP: Print Tree\nE: Exit\n");
        let mut power_input = String::new();
            io::stdin()
                .read_line(&mut power_input)
                .expect("Failed to read line");
        match power_input.trim() {
            "A" =>  a(&mut b_t),
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
                b_t.find(key);
            },
            "D" => {
                print!("Delete Not Implemented Yet\n");
            },
            "P" => b_t.print(),
            "E" => break,
            _ => print!("Please Input a Valid Arg\n"),
        }
        
        //b_t.root.print();
        //b_t.print();
        
}
}


fn a(n: &mut BinTree){
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
        n.add_node(Node{val: num});
    }
    //n.print();

}
