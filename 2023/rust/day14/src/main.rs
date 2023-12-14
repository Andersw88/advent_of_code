use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    println!();

    let input = include_str!("input.txt").lines();
    // let input = include_str!("example.txt");

    let height = input.clone().count();
    let width = input.clone().next().unwrap().len();

    let mut map = input
        .flat_map(|f| f.bytes())
        .collect_vec();

    let mut previous_maps: HashMap<Vec<u8>, usize> = Default::default();

    // Part 1
    // let directions: [(i32, i32); 1] = [(0, -1)];
    // let iterations = 1;

    // Part 2
    let directions: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let iterations = 1000000000;

    let mut j = 0;

    while j < iterations {
        for direction in directions {
            loop {
                let mut rocks_moved = false;
                (0..map.len()).for_each(|i| {
                    let x = i % width;
                    let y = i / width;
                    let x2 = x as i32 + direction.0;
                    let y2 = y as i32 + direction.1;
                    if x2 >= 0
                        && x2 < width as i32
                        && y2 >= 0
                        && y2 < height as i32
                        && map[x + y * width] == b'O'
                        && map[x2 as usize + y2 as usize * width] == b'.'
                    {
                        map.swap(x + y * width, x2 as usize + y2 as usize * width);
                        rocks_moved = true
                    }
                });
                if !rocks_moved {
                    break;
                }
            }
        }

        if let Some(j2) = previous_maps.get(&map) {
            let cycle = j - j2;
            println!("{:?}", (j2, j, cycle));
            j += ((iterations - j) / cycle) * cycle;
        } else {
            previous_maps.insert(map.clone(), j);
        }

        j += 1;
    }

    let result = map
        .iter()
        .enumerate()
        .filter_map(|(i, b)| {
            if *b == b'O' {
                Some(height - i / width)
            } else {
                None
            }
        })
        .sum::<usize>();

    println!("{:?}", result);
}
