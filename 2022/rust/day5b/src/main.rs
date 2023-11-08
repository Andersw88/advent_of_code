use std::{collections::{VecDeque}};
use itertools::Itertools;

fn main() {
    println!();

    let rows = include_str!("input.csv")
        .split('\n');
    let stack_rows = rows.clone().take_while(|row| {
        !row.starts_with(" 1")
    });

    let mut stacks: Vec::<VecDeque::<char>> = vec![VecDeque::from([]);stack_rows.clone().map(|row| row.len()).max().unwrap() / 4 + 1];
    
    stack_rows.clone().for_each(|row| {
        row.chars()
        .skip(1)
        .step_by(4)
        .enumerate()
        .filter(|(_, b)| *b != ' ')
        .for_each(|(index, block )| {
            stacks[index].push_front(block);
        })
    });

    let move_rows = rows.skip(stack_rows.count() + 2);
    move_rows.for_each(|row| 
    {
        let (amount,from,to) = row
            .splitn(6,' ')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect_tuple()
            .unwrap();

        let cur_len = stacks[from - 1].len();
        let popped = stacks[from - 1].drain(cur_len - amount..).collect_vec();
        stacks[to - 1].extend(popped);
    });

    for stack in stacks.clone()
    {
        println!("{:?}", stack);
    }

    print!("{:?}", stacks.iter().filter_map(|stack| stack.back()).join(""));

}
