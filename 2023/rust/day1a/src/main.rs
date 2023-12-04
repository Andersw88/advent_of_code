use itertools::Itertools;
fn main() {
    println!();

    let rows = include_str!("input.txt").split('\n');

    let result: i32 = rows.map(|row| {
        [row.chars().find(|c| c.is_numeric()).unwrap(),
         row.chars().rev().find(|c| c.is_numeric()).unwrap()].iter().join("").parse::<i32>().unwrap()
    }).sum();

    println!("{:?}", result);
}
