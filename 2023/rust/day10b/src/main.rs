use std::{
    cmp::{max, min},
    collections::HashSet,
};

use itertools::Itertools;

fn main() {
    println!();

    // let input = include_str!("example.txt").lines();
    // let input = include_str!("example3.txt").lines();
    let input = include_str!("input.txt").lines();

    let mut start = (0, 0);
    let pipe_map = input
        .clone()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '|' => [0_usize, 0_usize, 1_usize, 1_usize],
                    '-' => [1, 1, 0, 0],
                    'L' => [0, 1, 1, 0],
                    'J' => [1, 0, 1, 0],
                    '7' => [1, 0, 0, 1],
                    'F' => [0, 1, 0, 1],
                    'S' => {
                        start = (j, i);
                        [1, 1, 1, 1]
                    }
                    '.' => [0, 0, 0, 0],
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let height = input.clone().count();
    let width = pipe_map.len() / height;
    let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut expanded_map = vec![0; width * height];
    let mut open_list: Vec<(usize, usize, usize)> = [(start.0, start.1, 0)].into();

    while let Some(&next) = open_list.first() {
        open_list.remove(0);
        let pipe_next = pipe_map[next.0 + next.1 * width];

        expanded_map[next.0 + next.1 * width] = next.2 + 1;

        for (i, _) in pipe_next.iter().enumerate().filter(|(_, open)| **open == 1) {
            let direction = directions[i];
            let (x, y) = (next.0 as i32 + direction.0, next.1 as i32 + direction.1);

            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                let other_pipe = pipe_map[x as usize + y as usize * width];

                let other_direction_i = match i {
                    0 => 1,
                    1 => 0,
                    2 => 3,
                    3 => 2,
                    _ => unreachable!(),
                };

                if other_pipe[other_direction_i] == 1 && pipe_next[i] == 1 {
                    let expanded_step = &mut expanded_map[x as usize + y as usize * width];

                    if *expanded_step == 0 {
                        open_list.push((x as usize, y as usize, next.2 + 1));
                    }
                }
            }
        }
    }

    let pipe_map = input
        .clone()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if expanded_map[j + i * width] == 0 {
                        (j, '.')
                    } else {
                        (j, c)
                    }
                })
                .map(|(j, c)| match c {
                    '|' => [[0, 1, 0], 
                            [0, 1, 0], 
                            [0, 1, 0]],
                    '-' => [[0, 0, 0], 
                            [1, 1, 1], 
                            [0, 0, 0]],
                    'L' => [[0, 1, 0], 
                            [0, 1, 1], 
                            [0, 0, 0]],
                    'J' => [[0, 1, 0], 
                            [1, 1, 0], 
                            [0, 0, 0]],
                    '7' => [[0, 0, 0], 
                            [1, 1, 0], 
                            [0, 1, 0]],
                    'F' => [[0, 0, 0], 
                            [0, 1, 1], 
                            [0, 1, 0]],
                    'S' => {
                        start = (j, i);
                        [[0, 1, 0], [1, 1, 1], [0, 1, 0]]
                    }
                    '.' => [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let mut pipe_map_3x = vec![0; width * 3 * height * 3];

    pipe_map.iter().enumerate().for_each(|(n, pipe)| {
        pipe.iter().enumerate().for_each(|(i, pipe_row)| {
            pipe_row.iter().enumerate().for_each(|(j, pipe_section)| {
                pipe_map_3x[((i + (n / width) * 3) * width * 3) + (j + (n * 3) % (width * 3))] =
                    *pipe_section;
            })
        })
    });

    flood_fill(
        &mut pipe_map_3x,
        width as i32 * 3 / 2,
        height as i32 * 3 / 2,
        width * 3,
        height * 3,
    );

    // for line in pipe_map_3x.chunks(width * 3) {
    //     println!(
    //         "{:?}",
    //         line.iter()
    //             .map(|c| {
    //                 match *c {
    //                     2 => '*',
    //                     1 => '#',
    //                     _ => '.',
    //                 }
    //             })
    //             .join("")
    //     );
    // }

    let mut open_spaces_map = vec![true; pipe_map.len()];
    for (i, &value) in pipe_map_3x.iter().enumerate() {

        let x = i % (width * 3) / 3;
        let y = i / (width * 3) / 3;
        open_spaces_map[x + y * width] &= value == 2;
    };

    for line in open_spaces_map.chunks(width) {
        println!(
            "{:?}",
            line.iter()
            .map(|c| {
                match *c {
                    true => '*',
                    _ => '.',
                    }
                })
                .join("")
            );
        }
        
        let result = open_spaces_map.iter().filter(|b| **b).count();
        println!("{:?}", result);
}

fn flood_fill(pipe_map: &mut [i32], x: i32, y: i32, width: usize, height: usize) {
    let mut to_fill = vec![(x, y)];
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // Possible movement directions

    while let Some((x, y)) = to_fill.pop() {
        if x >= 0
            && x < width as i32
            && y >= 0
            && y < height as i32
            && pipe_map[(y * width as i32 + x) as usize] != 0
        {
            continue;
        }
        pipe_map[(x + y * width as i32) as usize] = 2;

        for direction in &directions {
            let x2 = x + direction.0;
            let y2 = y + direction.1;

            to_fill.push((x2, y2));
        }
    }
}
