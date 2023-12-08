use std::collections::HashMap;

use itertools::Itertools;


fn main() {
    println!();

    // let mut input = include_str!("example.txt").lines();
    let mut input = include_str!("input.txt").lines();

    let sequence = input.next().unwrap().chars().map(|c| (c == 'R') as usize).collect_vec();

    let hash_map: HashMap<String, [String;2]> = input.skip(1).map(|line| { 

        (line[0..3].to_string(),[line[7..10].to_string(), line[12..15].to_string()])
    }).collect();


    let mut curr_position = "AAA".to_string();
    let goal = "ZZZ";
    let result = sequence.iter().cycle().map(|&c| {
        curr_position = hash_map[&curr_position][c].to_owned();
        curr_position != goal
    }).take_while(|&b| b).count() + 1;
    println!("{result:?}");
}
