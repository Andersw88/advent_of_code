use std::collections::HashSet;

struct Card {
    copies: i64,
    winning_numbers: HashSet<u32>,
    your_numbers: HashSet<u32>,
}

fn parse_card(line: &str) -> Card {
    let parts: Vec<&str> = line.split(": ").collect();
    let card_number: u32 = parts[0].split_whitespace().nth(1).unwrap().parse().unwrap();

    let numbers: Vec<&str> = parts[1].split(" | ").collect();
    let winning_numbers: HashSet<u32> = numbers[0]
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    let your_numbers: HashSet<u32> = numbers[1]
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    assert!(your_numbers.len() == 25 || your_numbers.len() == 8);

    Card {
        copies: 1,
        winning_numbers,
        your_numbers,
    }
}

fn main() {
    println!();

    let input = include_str!("example.txt");
    // let input = include_str!("input.txt");

    let mut cards: Vec<Card> = vec![];

    for info in input.lines() {
        let card = parse_card(info);
        cards.push(card);
    }

    let result: i64 = cards.iter().map(|card| {
        let wins = card.your_numbers.intersection(&card.winning_numbers).count() as u32;

        (1 << wins) / 2
    }).sum();
    println!("{}", result);
    
}