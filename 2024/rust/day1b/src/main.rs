use std::{collections::HashMap, iter::Zip};

use itertools::Itertools;
fn main() {
    println!();

    let rows = include_str!("input.txt").split('\n');
    // let rows = include_str!("example.txt").split('\n');

    let mut m1: HashMap<i32, i32> = HashMap::new();
    let mut m2: HashMap<i32, i32> = HashMap::new();
    rows.clone().for_each(|row| {
        let mut parts = row.split("   ");
        let num = parts.next()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        *m1.entry(num).or_default() += 1;
    });

    rows.for_each(|row| {
        let mut parts = row.split("   ");
        let second = parts
        .nth(1)
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();
        *m2.entry(second).or_default() += 1;
    });

    let result = m1.clone().iter().fold(0, | sum, (n1, n2) | {
        let n3 = *m2.entry(*n1).or_default();
        sum + n1 * n2 * n3 
    });

    println!("{:?}", result);
}
