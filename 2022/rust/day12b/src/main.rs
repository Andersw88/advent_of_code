use std::{cmp::Ordering, collections::BinaryHeap, rc::Rc};

use itertools::Itertools;
use ordered_float::OrderedFloat;

#[derive(Eq, Debug)]
struct Node {
    index: usize,
    x: i32,
    y: i32,
    current_cost: usize,
    estimated_cost: OrderedFloat<f64>,
    history: Option<Rc<Node>>,
}

impl Node {
    fn new(index: usize, width: usize) -> Node {
        Self::new_linked(
            index,
            width,
            0,
            OrderedFloat(f64::INFINITY),
            Default::default(),
        )
    }
    fn new_linked(
        index: usize,
        width: usize,
        current_cost: usize,
        estimated_cost: OrderedFloat<f64>,
        old_node: Option<Rc<Node>>,
    ) -> Node {
        Node {
            index,
            x: (index % width) as i32,
            y: (index / width) as i32,
            current_cost,
            estimated_cost,
            history: old_node,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.estimated_cost.cmp(&other.estimated_cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

struct AStar {
    open_list: BinaryHeap<Rc<Node>>,
    closed_list: Vec<f64>,
}

impl AStar {
    fn new() -> AStar {
        AStar {
            open_list: Default::default(),
            closed_list: Default::default(),
        }
    }
}

fn expand_node(
    closed_list: &[f64],
    node: &Rc<Node>,
    map: &[u8],
    width: usize,
    height: usize,
    x_goal: usize,
    y_goal: usize,
) -> Vec<Node> {
    [
        (node.x - 1, node.y),
        (node.x + 1, node.y),
        (node.x, node.y - 1),
        (node.x, node.y + 1),
    ]
    .iter()
    .filter_map(|coord| {
        let index = coord.0 + coord.1 * width as i32;
        let new_node_cost = node.current_cost + 1;
        if coord.0 >= 0
            && coord.0 < width as i32
            && coord.1 >= 0
            && coord.1 < height as i32
            && map[coord.0 as usize + coord.1 as usize * width]
                <= map[node.x as usize + node.y as usize * width] + 1
        {
            let heuristic = calc_heuristic(coord.0 as usize, coord.1 as usize, x_goal, y_goal);
            let estimated_cost = OrderedFloat(new_node_cost as f64 + heuristic);
            if *estimated_cost < closed_list[index as usize] {
                Some(Node::new_linked(
                    index as usize,
                    width,
                    new_node_cost,
                    estimated_cost,
                    Some(node.clone()),
                ))
            } else {
                None
            }
        } else {
            None
        }
    })
    .collect_vec()
}

fn calc_heuristic(x: usize, y: usize, x_goal: usize, y_goal: usize) -> f64 {
    x.abs_diff(x_goal) as f64 + y.abs_diff(y_goal) as f64
    // ((x.abs_diff(x_goal) * x.abs_diff(x_goal)) as f64
    //     + (y.abs_diff(y_goal) * y.abs_diff(y_goal)) as f64)
    //     .sqrt()
}

fn main() {
    println!();

    let input = include_str!("input.csv").split('\n');
    // let input = include_str!("example.csv").split('\n');
    let width = input.clone().next().unwrap().len();
    let height = input.clone().count();

    let mut start_index = 0;
    let mut end_index = 0;

    // input.clone().for_each(|f| {
    //     println!("{:?}", f);
    // });

    let map = input
        .flat_map(|f| f.bytes())
        .enumerate()
        .map(|(i, c)| match c {
            b'S' => {
                start_index = i;
                0
            }
            b'E' => {
                end_index = i;
                b'z' - b'a'
            }
            c => c - b'a',
        })
        .collect_vec();

    let mut astar = AStar::new();
    astar.open_list.push(Node::new(start_index, width).into());
    astar.closed_list = vec![f64::INFINITY; map.len()];

    let x_goal = end_index % width;
    let y_goal = end_index / width;

    let mut final_node = astar.open_list.peek().unwrap().clone();
    while let Some(node) = astar.open_list.pop() {
        if node.index == end_index {
            final_node = node;
            break;
        }
        let new_nodes = expand_node(
            &astar.closed_list,
            &node,
            &map,
            width,
            height,
            x_goal,
            y_goal,
        );

        new_nodes.iter().for_each(|node| {
            astar.closed_list[node.index] = *node.estimated_cost;
        });

        astar
            .open_list
            .extend(new_nodes.into_iter().map(|new_node| new_node.into()));
    }

    let mut node_vec = Vec::new();
    let mut current = Some(final_node.clone());

    while let Some(ref curr) = current {
        node_vec.push(curr.clone());
        current = match &curr.history {
            Some(_) => curr.history.clone(),
            None => None,
        };
    }
    node_vec.reverse();

    let mut map2 = vec![0; map.len()];
    node_vec
        .iter()
        .for_each(|node| map2[node.x as usize + node.y as usize * width] = 255);

    println!();
    map2.iter()
        .enumerate()
        .map(|(i, f)| {
            if *f == 255 {
                '#'
            } else {
                (map[i] + b'a') as char
            }
        })
        .collect_vec()
        .chunks(width)
        .for_each(|f| {
            println!("{:?}", f.iter().join(""));
        });

    println!("{:?}", node_vec.len() - 1);
}
