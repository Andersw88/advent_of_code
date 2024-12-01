use itertools::Itertools;
fn main() {
    let rows = include_str!("input.txt").split('\n');
    // let rows = include_str!("example.txt").split('\n');

    let n1 = rows
        .clone()
        .map(|row| {
            let mut parts = row.split("   ");
            return parts.next().unwrap().trim().parse::<i32>().unwrap();
        })
        .sorted()
        .collect_vec();

    let n2 = rows
        .map(|row| {
            let mut parts = row.split("   ");
            return parts.nth(1).unwrap().trim().parse::<i32>().unwrap();
        })
        .sorted()
        .collect_vec();

    let result = n1
        .iter()
        .zip(n2)
        .fold(0, |sum, (n1, n2)| sum + i32::abs(n1 - n2));

    println!("{:?}", result);
}
