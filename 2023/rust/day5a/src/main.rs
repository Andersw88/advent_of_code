use itertools::Itertools;

fn in_range(start_dest: i64, start_src: i64, range_length: i64, num: i64) -> Option<i64> {
    if num >= start_src && num < start_src + range_length {
        let offset = num - start_src;

        Some(start_dest + offset)
    } else {
        None
    }
}

fn main() {
    println!();

    let mut input = include_str!("input.txt").lines();
    // let mut input = include_str!("example.txt").lines();

    let mut seeds = input
        .next()
        .map(|row: &str| {
            row.split_whitespace()
                .skip(1)
                .map(|part| part.parse::<i64>().unwrap())
                .collect_vec()
        })
        .unwrap();

    // println!("{:?}", seeds);

    let mut next_category_seeds: Vec<i64> = seeds.to_owned();
    input.for_each(|row| {
        let parts: Vec<&str> = row.split_whitespace().collect();

        match parts.len() {
            2 => {
                println!("{:?}", parts[0]);
            }
            3 => {
                let values: Vec<i64> = parts.iter().map(|s| s.parse().unwrap()).collect();

                seeds.iter().enumerate().for_each(|(i, seed)| {
                    if let Some(new_id) = in_range(values[0], values[1], values[2], *seed) {
                        next_category_seeds[i] = new_id;
                    }
                });
            }
            _ => {
                seeds = next_category_seeds.clone();
                println!("{:?}", seeds);
            }
        }
    });

    seeds = next_category_seeds.clone();

    let result = seeds.iter().min().unwrap();

    println!("{:?}", result);
}
