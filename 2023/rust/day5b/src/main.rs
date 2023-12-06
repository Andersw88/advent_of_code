use itertools::Itertools;

fn main() {
    println!();

    let mut input = include_str!("input.txt").lines();
    // let mut input = include_str!("example.txt").lines();

    let seeds_start_range = input
        .next()
        .map(|row: &str| {
            row.split_whitespace()
                .skip(1)
                .map(|part| part.parse::<usize>().unwrap())
                .chunks(2)
                .into_iter()
                .map(|mut parts| parts.next_tuple::<(usize, usize)>().unwrap())
                .map(|(start, range)| (start, start + range))
                .collect_vec()
        })
        .unwrap();

    let max = seeds_start_range.iter().map(|(_, max)| max).max().unwrap() * 2;

    let mut seeds = vec![0_u8; max];

    seeds_start_range
        .iter()
        .flat_map(|(min, max)| *min..*max)
        .for_each(|i| {
            seeds[i] = 1;
        });

    let mut next_category_seeds: Vec<u8> = vec![0_u8; max];
    input.skip(1).for_each(|row| {
        let parts: Vec<&str> = row.split_whitespace().collect();

        match parts.len() {
            2 => {
                println!("{:?}", parts[0]);
            }
            3 => {
                let values: Vec<usize> = parts.iter().map(|s| s.parse().unwrap()).collect();
                let start_dest = values[0];
                let start_src = values[1];
                let range_length = values[2];

                (start_src..start_src + range_length).for_each(|i| {
                    if seeds[i] == 1 {
                        seeds[i] = 0;
                        let new_id = start_dest + i - start_src;
                        next_category_seeds[new_id] = 1;
                    }
                });
            }
            _ => {
                seeds = next_category_seeds
                    .iter()
                    .zip(seeds.iter())
                    .map(|(&c, &s)| (c == 1 || s == 1) as u8)
                    .collect_vec();
                next_category_seeds.fill(0);
            }
        }
    });

    seeds = next_category_seeds
        .iter()
        .zip(seeds.iter())
        .map(|(&c, &s)| (c == 1 || s == 1) as u8)
        .collect_vec();
    let result = seeds.iter().find_position(|c| **c > 0).unwrap().0;

    println!("{:?}", result);
}
