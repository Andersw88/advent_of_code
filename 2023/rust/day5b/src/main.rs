use std::mem::swap;
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
    // let mut seeds_bc: Vec<u8> = seeds.clone();
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


                (0_usize..max).for_each(|i| {
                    if seeds[i] > 0 {
                        if i >= start_src && i < start_src + range_length {
                            if next_category_seeds[i] == 2 {
                                next_category_seeds[i] = 0;
                            }
                            seeds[i] = 0;
                            let new_id = start_dest + i - start_src;
                            next_category_seeds[new_id] = 1;
                        } else if next_category_seeds[i] != 1 {

                            next_category_seeds[i] = 2;
                        }
                    }
                });
            }
            _ => {
                // let before = seeds_bc.iter().filter(|c| **c > 0).count();
                // let after = next_category_seeds.iter().filter(|c| **c > 0).count();
                // println!("{:?}", seeds_bc.iter().map(|c| (*c + b'0') as char).join(""));
                // println!("{:?}", seeds.iter().map(|c| (*c + b'0') as char).join(""));
                // println!("{:?}", next_category_seeds.iter().map(|c| (*c + b'0') as char).join(""));
                // assert!(before == after);
                seeds = next_category_seeds.iter().map(|c| (*c > 0) as u8).collect_vec();
                next_category_seeds.fill(0);
            }
        }
    });

    swap(&mut seeds, &mut next_category_seeds);
    let result = seeds.iter().find_position(|c| **c > 0).unwrap().0;

    println!("{:?}", result);
}
