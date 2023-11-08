use std::{collections::HashSet};
use itertools::Itertools;

fn main() {
    println!();
    let cals = include_str!("input.csv")
        .split('\n')
        .map(|row| 
            row.splitn(4, |s| s == '-' || s == ',').map(|f| f.parse::<i32>().unwrap()).collect_tuple().unwrap()
        )
        .map(|(s1,e1,s2,e2)| {
            let first_range: HashSet<i32> = (s1..=e1).collect();
            let second_range: HashSet<i32> = (s2..=e2).collect();
            first_range.is_superset(&second_range) || first_range.is_subset(&second_range)
        }).filter(|p| *p)
        .count();

    println!("{:?}", cals);
}
