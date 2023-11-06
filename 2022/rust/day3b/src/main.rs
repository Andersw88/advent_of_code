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
    let cals = include_str!("input.csv").split('\n').chunks(3);

    let cals2: u32 = cals
        .into_iter()
        .map(|mut chunk| {
            let mut rucksack1: HashSet<char> = chunk.next().unwrap().chars().collect();
            let rucksack2: HashSet<char> = chunk.next().unwrap().chars().collect();
            let rucksack3: HashSet<char> = chunk.next().unwrap().chars().collect();

            rucksack1.retain(|f| rucksack2.contains(f) && rucksack3.contains(f));

            rucksack1
                .iter()
                .map(|c| char_to_priority(*c as u8) as u32)
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("{:?}", cals2);
}
