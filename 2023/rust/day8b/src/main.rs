use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    println!();

    // let mut input = include_str!("example.txt").lines();
    let mut input = include_str!("input.txt").lines();

    let sequence = input
        .next()
        .unwrap()
        .chars()
        .map(|c| (c == 'R') as usize)
        .collect_vec();

    let mut starting_positions: Vec<String> = Default::default();
    let mut goal_map: HashMap<String, Vec<usize>> = Default::default();

    let hash_map: HashMap<String, [(String, bool); 2]> = input
        .skip(1)
        .map(|line| {
            let first_part = line[0..3].to_string();
            let first_part_is_start = line.as_bytes()[2] == b'A';
            if first_part_is_start {
                starting_positions.push(first_part.clone())
            }

            let second_part = line[7..10].to_string();
            let second_part_is_goal = line.as_bytes()[9] == b'Z';
            if second_part_is_goal {
                goal_map.insert(second_part.clone(), Default::default());
            }
            let third_part = line[12..15].to_string();
            let third_part_is_goal = line.as_bytes()[14] == b'Z';
            if third_part_is_goal {
                goal_map.insert(third_part.clone(), Default::default());
            }

            (
                first_part,
                [
                    (second_part.to_string(), second_part_is_goal),
                    (third_part, third_part_is_goal),
                ],
            )
        })
        .collect();

    for (i, &c) in sequence.iter().cycle().enumerate() {
        // println!("starting_positions:{:?}", starting_positions);
        starting_positions
            .iter_mut()
            .for_each(|current_position: &mut String| {
                let curr = &hash_map[current_position][c];
                *current_position = (*curr.0).to_string();

                if curr.1 {
                    goal_map
                        .entry(current_position.to_string())
                        .and_modify(|p| p.push(i + 1));
                }
            });
        if goal_map.values().all(|seen_vec| seen_vec.len() >= 2) {
            break;
        }
    }

    let cycles: Vec<u64> = goal_map
        .values()
        .map(|seen_vec| (seen_vec[0]) as u64)
        .collect_vec();

    println!(
        "{:?}",
        cycles.iter()
            .flat_map(|c| divisors::get_divisors(*c))
            .unique()
            .product::<u64>()
    );
}
