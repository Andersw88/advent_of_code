use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Eq, Debug)]
struct Hand
{
    hand: Vec<i32>,
    hand_score: i32,
    bid: i32,
}

fn hand_strength(hand: &Vec<i32>) -> i32
{
    let mut hand_counts = [0; 15];
    for &item in hand {
        hand_counts[item as usize] += 1;
    }
    let hand_counts_sorted = hand_counts[2..hand_counts.len()].iter().sorted_by(|a, b| b.cmp(a)).collect_vec();

    match hand_counts_sorted[0] + hand_counts[1]{
        5 => 7,
        4 => 6,
        3 => 
            match hand_counts_sorted[1] {
                2 => 5,
                _ => 4
            },
        2 => 
            match hand_counts_sorted[1] {
                2 => 3,
                _ => 2
            },
        _ => 1
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.hand_score == other.hand_score
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.hand_score, &self.hand).cmp(&(other.hand_score, &other.hand))
    }
}

fn char_to_strength(c: char) -> i32
{
    match c
    {
        c if c.is_ascii_digit() => {c.to_digit(10).unwrap() as i32},
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => panic!()
    }
}

fn main() {
    println!();

    // let input = include_str!("example.txt").lines();
    let input = include_str!("input.txt").lines();

    let mut hand_vec = input.map(|line| line.split_once(' ')
    .map(|(shand, sbid)| 
    {
        let hand = shand.chars().map(char_to_strength).collect_vec();
        let bid = sbid.parse::<i32>().unwrap();

        let hand_score = hand_strength(&hand);
        Hand {
            hand: hand.clone(),
            hand_score,
            bid
        }
    }).unwrap()).collect_vec();

    hand_vec.sort();

    hand_vec.iter().for_each(|hand| {
        println!("{hand:?}");
    });

    let result: i32 = hand_vec.iter().enumerate().map(|(rank, hand)| {
        (rank + 1) as i32 * hand.bid
    }).sum();

    println!("{result:?}");
}
