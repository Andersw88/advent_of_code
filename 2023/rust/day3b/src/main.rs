use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn is_valid_char(c: char) -> bool {
    c == '*'
}

fn parse_input(input: &str) -> (HashSet<(i32, i32)>, HashMap<(i32, i32), i32>) {
    let mut hash_set: HashSet<(i32, i32)> = Default::default();
    let mut hash_map: HashMap<(i32, i32), i32> = Default::default();

    for (row_idx, line) in input.lines().enumerate() {
        let mut current_number: i32 = 0;
        let mut current_number_length: usize = 0;

        for (col_idx, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                let num: i32 = c.to_digit(10).unwrap() as i32;
                current_number = current_number * 10 + num;
                current_number_length += 1;

                if col_idx == line.len() - 1 {
                    (1..=current_number_length).for_each(|xx| {
                        hash_map.insert(((col_idx - xx) as i32, row_idx as i32), current_number);
                    });
                }
            } else {
                if current_number != 0 {
                    (1..=current_number_length).for_each(|xx| {
                        hash_map.insert(((col_idx - xx) as i32, row_idx as i32), current_number);
                    });
                }
                current_number = 0;
                current_number_length = 0;

                if is_valid_char(c) {
                    hash_set.insert((col_idx as i32, row_idx as i32));
                }
            }
        }
    }
    (hash_set, hash_map)
}

fn main() {
    println!();

    // let input = include_str!("example.txt");
    let input = include_str!("input.txt");
    let (hash_set, hash_map) = parse_input(input);

    let result: i32 = hash_set
        .iter()
        .flat_map(|(col, row)| {
            let mut first_gear: Option<i32> = None;
            (-1..=1)
                .cartesian_product(-1..=1)
                .filter_map(|(x1, y1)| {
                    let x = col + x1;
                    let y = row + y1;
                    if let Some(&gear_value) = hash_map.get(&(x, y)) {
                        // println!("1:{:?}", (gear_value, x, y));
                        if let Some(gear) = first_gear {
                            if gear_value != gear {
                                // println!("2:{:?}", (gear, gear_value));
                                return Some(gear * gear_value);
                            }
                        }
                        first_gear = Some(gear_value);
                    }
                    None
                })
                .take(1)
                .collect::<Vec<_>>()
        })
        .sum();

    println!("{:?}", result);
}
