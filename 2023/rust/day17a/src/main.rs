use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    rc::Rc,
};

#[derive(Eq, Debug)]
struct Node {
    x: i32,
    y: i32,
    direction: (i32, i32),
    steps: i32,
    current_cost: usize,
    history: Option<Rc<Node>>,
}

impl Node {
    fn new(x: i32, y: i32, direction: (i32, i32)) -> Node {
        Self::new_linked(x, y, direction, 0, 0, Default::default())
    }
    fn new_linked(
        x: i32,
        y: i32,
        direction: (i32, i32),
        steps: i32,
        current_cost: usize,
        old_node: Option<Rc<Node>>,
    ) -> Node {
        Node {
            x,
            y,
            direction,
            steps,
            current_cost,
            history: old_node,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        (self.x, self.y, self.direction, self.steps)
            == (other.x, other.y, other.direction, other.steps)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.current_cost.cmp(&other.current_cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

fn main() {
    println!();

    let input = include_str!("input.txt").lines();
    // let input = include_str!("example.txt").lines();

    let height = input.clone().count();
    let width = input.clone().next().unwrap().len();

    let map = input
        .flat_map(|f| f.bytes().map(|c| c - b'0'))
        .collect_vec();

    let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut open_list: BinaryHeap<Rc<Node>> = Default::default();

    let mut explored_map: HashSet<(i32, i32, (i32, i32), i32)> = Default::default();

    let goal = (width as i32 - 1, height as i32 - 1);

    open_list.push(Node::new(0, 0, (1, 0)).into());

    let mut result: Option<Rc<Node>> = None;

    while let Some(node) = open_list.pop() {

        let explored_key = (node.x, node.y, node.direction, node.steps);

        if explored_map.contains(&explored_key) {
            continue;
        }
        explored_map.insert(explored_key);

        if node.x == goal.0 && node.y == goal.1 {
            result = Some(node.clone());
            break;
        }

        let new_beems: Vec<Rc<Node>> = directions
            .iter()
            .filter_map(|dir| {
                let (nx, ny) = (node.x + dir.0, node.y + dir.1);

                let new_steps = if *dir == node.direction {
                    node.steps + 1
                } else {
                    0
                };

                // Stop reverseing.
                if match dir {
                    (1, 0) => node.direction == (-1, 0),
                    (-1, 0) => node.direction == (1, 0),
                    (0, 1) => node.direction == (0, -1),
                    (0, -1) => node.direction == (0, 1),
                    _ => unreachable!(),
                } {
                    return None;
                }

                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 && new_steps < 3 {
                    let cost = node.current_cost + map[nx as usize + ny as usize * width] as usize;

                    return Some(Rc::new(Node::new_linked(
                        nx,
                        ny,
                        *dir,
                        new_steps,
                        cost,
                        Some(node.clone()),
                    )));
                }
                None
            })
            .collect_vec();

        new_beems.into_iter().for_each(|node| {
            let explored_key = (node.x, node.y, node.direction, node.steps);
            if !explored_map.contains(&explored_key) {
                open_list.push(node);
            } 
        });
    }

    let mut map2 = map.iter().map(|b| (b + b'0') as char).collect_vec();

    let mut hist = Some(result.clone().unwrap());
    while let Some(node) = hist {
        map2[node.x as usize + node.y as usize * width] = '*';

        hist = node.history.clone();
    }
    
    map2.chunks(width).for_each(|row| {
        println!("{:?}", row.iter().join(" "));
    });

    println!();

    println!("result: {:?}", result.clone().unwrap());
    println!("max: {:?}", result.unwrap().current_cost)
}
