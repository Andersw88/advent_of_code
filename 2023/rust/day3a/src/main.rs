use std::collections::HashSet;

use itertools::Itertools;

fn is_valid_char(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn parse_input(input: &str) -> (Vec<(u32, usize, usize, usize)>, HashSet<(i32, i32)>) {
    let mut hash_set: HashSet<(i32, i32)> = Default::default();
    let mut numbers_with_coords: Vec<(u32, usize, usize, usize)> = Vec::new();

    for (row_idx, line) in input.lines().enumerate() {
        let mut current_number: u32 = 0;
        let mut current_number_position: Option<(usize, usize)> = None;
        let mut current_number_length: usize = 0;

        for (col_idx, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if current_number_length == 0 {
                    current_number_position = Some((row_idx, col_idx));
                }

                let num: u32 = c.to_digit(10).unwrap();
                current_number = current_number * 10 + num;
                current_number_length += 1;

                if col_idx == line.len() - 1 {
                    numbers_with_coords.push((
                        current_number,
                        current_number_position.unwrap().0,
                        current_number_position.unwrap().1,
                        current_number_length,
                    ));
                }
            } else {
                if current_number != 0 {
                    numbers_with_coords.push((
                        current_number,
                        current_number_position.unwrap().0,
                        current_number_position.unwrap().1,
                        current_number_length,
                    ));
                }
                current_number = 0;
                current_number_position = None;
                current_number_length = 0;

                if is_valid_char(c) {
                    hash_set.insert((row_idx.try_into().unwrap(), col_idx.try_into().unwrap()));
                }
            }
        }
    }

    (numbers_with_coords, hash_set)
}

fn main() {
    println!();

    // let input = include_str!("example.txt");
    let input = include_str!("input.txt");

    let (numbers_with_coords, hash_set) = parse_input(input);

    let numbers2 = numbers_with_coords
        .iter()
        .map(|(number, row, col, digits)| {
            (0..*digits)
                .map(|digit| digit + *col)
                .map(|curr_col| {
                    (-1..=1)
                        .cartesian_product(-1..=1)
                        .map(|(x1, y1)| {
                            let x = *row as i32 + x1;
                            let y = curr_col as i32 + y1;
                            hash_set.contains(&(x, y))
                        })
                        .any(|c| c)
                })
                .any(|c| c)
                .then_some(number)
        })
        .collect_vec();

    let sum = numbers2.iter().filter_map(|&f| f).sum::<u32>();

    println!("{:?}", numbers2.iter().filter_map(|&f| f).collect_vec());
    println!("{:?}", sum);
}
