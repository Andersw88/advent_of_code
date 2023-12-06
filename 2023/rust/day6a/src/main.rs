use itertools::Itertools;

fn main() {
    println!();

    let input = include_str!("input.txt").lines();
    // let input = include_str!("example.txt").lines();

    let (times, distances) = input
        .map(|line| {
            line.trim_start_matches("Time:    ")
                .trim_start_matches("Distance:")
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_tuple()
        .unwrap();

    let result = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| {
            (1..time)
                .zip((1..time).rev())
                .filter(|(time1, time2)| time1 * time2 > distance)
                .count()
        })
        .product::<usize>();

    println!("{result:?}")
}
