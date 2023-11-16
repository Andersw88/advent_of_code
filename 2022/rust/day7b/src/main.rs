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

    pub fn min_size_to_delete(&self, space_to_free: i32, curr_size_to_delete: i32 ) -> i32
    {   
        let min_child = self.children.iter().fold(curr_size_to_delete, |curr_size_to_delete, (_, child)| 
        {
            let child_size = child.borrow().min_size_to_delete(space_to_free, curr_size_to_delete);
            if child_size < curr_size_to_delete { child_size } else { curr_size_to_delete }
        });
        let return_size_to_delete = if self.size > space_to_free && self.size < curr_size_to_delete { self.size } else {curr_size_to_delete};

        if min_child > space_to_free && min_child < return_size_to_delete { min_child } else {return_size_to_delete}
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

    let unused = 70_000_000 - total_size;
    let space_required = 30_000_000 - unused;

    let result = root_dir.borrow_mut().min_size_to_delete(space_required, total_size);

    println!("{:?}, {:?}", total_size, space_required);
    println!("{:?}", result);

}
