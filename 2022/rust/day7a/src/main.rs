use std::{collections::HashMap, rc::{Rc, Weak}, cell::RefCell, i32};

#[derive(Debug)]
struct Node
{
    children: HashMap::<String, Rc<RefCell<Node>>>,
    parent: Weak<RefCell<Node>>,
    size: i32,
}

impl Node {
    pub fn new(parent: Weak<RefCell<Node>>) -> Node {
        Node {
            children: HashMap::new(),
            parent,
            size: 0
        }
    }
    pub fn new_root() -> Node {
        Node {
            children: HashMap::new(),
            parent: Weak::new(),
            size: 0
        }
    }

    pub fn sum_children(&mut self) -> i32 {
        self.size += self.children.iter().fold(0, |acc, (_, child)| acc + child.borrow_mut().sum_children());
        self.size
    }

    pub fn count_sizes(&self) -> i32
    {   
        self.children.iter().fold(0, |acc, (_, child)| 
        {
            let child_size = child.borrow().count_sizes();
            acc + child_size
        }) + if self.size <= 100000 { self.size }  else { 0 }
    }    
}


fn main() {
    println!();
    let root_dir: Rc<RefCell<Node>> = Rc::new(Node::new_root().into());
    let mut current_dir = root_dir.clone();

    // include_str!("example.csv")
    include_str!("input.csv")
        .split('\n')
        .for_each(|command| {
        match command {
            _ if command.starts_with("$ cd /") => {
                current_dir = root_dir.clone();
            }
            _ if command.starts_with("$ cd ..") => {
                if let Some(parent) = {current_dir.clone()}.borrow().parent.upgrade() {
                    current_dir = parent.clone();
                }
            }
            c if command.starts_with("$ cd") => {
                let folder_name = c.split(' ').last().unwrap();
                current_dir = {current_dir.clone()}.borrow().children[folder_name].clone();

            }
            c if command.starts_with("dir") => {
                let folder_name = c.split(' ').last().unwrap();
                current_dir.borrow_mut().children.insert(folder_name.to_owned(), Rc::new(Node::new(Rc::downgrade(&current_dir)).into()));
            }
            _ if command == "$ ls" => {}
            c => {
                let size: i32 = c.split(' ').next().unwrap().parse().unwrap();
                current_dir.borrow_mut().size += size;
            }
        }
    });
    let total_size = root_dir.borrow_mut().sum_children();
    let result = root_dir.borrow_mut().count_sizes();

    println!("{:?}", total_size);
    println!("{:?}", result);

}
