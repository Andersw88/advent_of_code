
use std::collections::HashSet;

fn main() {
    println!();

    let input = include_str!("input.csv").split('\n');
    // let input = include_str!("example.csv").split('\n');

    let mut hash_set: HashSet<(i32,i32)> = HashSet::new();
    let mut head = (0,0);
    let mut tail = (0,0);
    hash_set.insert(tail);

    input.filter_map(|f| f.split_once(' '))
    .filter_map(|(direction_str, steps_str)| {
    steps_str.parse::<i32>().ok().and_then(|steps| {
        match direction_str {
            "R" => Some((1, 0)),
            "L" => Some((-1,0)),
            "U" => Some((0, 1)),
            "D" => Some((0,-1)),
            _ => None,
        }.map(|direction | (direction, steps))
    })
    })
    .for_each(|(direction, steps)| {
        for _ in 0..steps {
            head = (head.0 + direction.0, head.1 + direction.1);
            if i32::abs(head.0 - tail.0) > 1 || i32::abs(head.1 - tail.1) > 1
            {
                tail = (tail.0 + i32::signum(head.0 - tail.0), tail.1 + i32::signum(head.1 - tail.1));
                hash_set.insert(tail);
            }
            println!("{:?},{:?}", head, tail)
        }
    });
    
    println!("{:?}", hash_set.len());


}
