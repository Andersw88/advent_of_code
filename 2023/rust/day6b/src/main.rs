use itertools::Itertools;

fn main() {
    println!();

    let input = include_str!("input.txt").lines();

    let (time, distance) = input
        .map(|line| {
            line.trim_start_matches("Time:    ")
                .trim_start_matches("Distance:")
                .replace(' ', "")
                .parse::<i64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    let result = (1..time)
        .zip((1..time).rev())
        .filter(|(time1, time2)| time1 * time2 > distance)
        .count();

    println!("{result:?}")
}
