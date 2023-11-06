use itertools::Itertools;
use std::collections::HashSet;

fn char_to_priority(char_value: u8) -> u8 {
    if char_value > b'Z' {
        return char_value - b'a' + 1;
    }
    char_value - b'A' + 27
}

fn main() {
    println!();
    let cals: u32 = include_str!("input.csv")
        .split('\n')
        .map(|rucksack| {
            let find_chars: HashSet<char> = rucksack[..rucksack.len() / 2].chars().collect();

            rucksack[rucksack.len() / 2..]
                .chars()
                .filter(|character| find_chars.contains(character))
                .unique()
                .map(|c| char_to_priority(c as u8) as u32)
                .sum::<u32>()
        })
        .sum();

    println!("{:?}", cals);
}
