use itertools::{izip, Itertools};
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum D {
    Right = 0,
    Left,
    Down,
    Up,
}

fn main() {
    println!();

    let input = include_str!("input.txt").lines();
    // let input = include_str!("example.txt").lines();

    let height = input.clone().count();
    let width = input.clone().next().unwrap().len();

    let map = input.flat_map(|f| f.bytes()).collect_vec();

    let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    // Part 1
    // let beem_start_positions = [(0,0,D::Right)];

    // Part 2
    let beem_start_positions = izip!(
        (0..width as i32),
        std::iter::repeat(0_i32),
        std::iter::repeat(D::Down)
    )
    .chain(izip!(
        (0..width as i32),
        std::iter::repeat(width as i32 - 1_i32),
        std::iter::repeat(D::Up)
    ))
    .chain(izip!(
        std::iter::repeat(0_i32),
        0..height as i32,
        std::iter::repeat(D::Right)
    ))
    .chain(izip!(
        std::iter::repeat(height as i32 - 1_i32),
        0..height as i32,
        std::iter::repeat(D::Left)
    ))
    .collect_vec();

    let result = beem_start_positions
        .iter()
        .map(|beem_start| {
            let mut beem_positions = vec![*beem_start];
            let mut explored_map = HashSet::<(i32, i32, D)>::new();
            let mut energized_map = HashSet::<(i32, i32)>::new();

            explored_map.insert(beem_positions[0]);
            energized_map.insert((beem_positions[0].0, beem_positions[0].1));

            while let Some(beem_position) = beem_positions.pop() {
                let new_beems =
                    match map[beem_position.0 as usize + beem_position.1 as usize * width] {
                        b'\\' => match beem_position.2 {
                            D::Right => vec![(
                                beem_position.0 + directions[D::Down as usize].0,
                                beem_position.1 + directions[D::Down as usize].1,
                                D::Down,
                            )],
                            D::Left => vec![(
                                beem_position.0 + directions[D::Up as usize].0,
                                beem_position.1 + directions[D::Up as usize].1,
                                D::Up,
                            )],
                            D::Down => vec![(
                                beem_position.0 + directions[D::Right as usize].0,
                                beem_position.1 + directions[D::Right as usize].1,
                                D::Right,
                            )],
                            D::Up => vec![(
                                beem_position.0 + directions[D::Left as usize].0,
                                beem_position.1 + directions[D::Left as usize].1,
                                D::Left,
                            )],
                        },
                        b'/' => match beem_position.2 {
                            D::Right => vec![(
                                beem_position.0 + directions[D::Up as usize].0,
                                beem_position.1 + directions[D::Up as usize].1,
                                D::Up,
                            )],
                            D::Left => vec![(
                                beem_position.0 + directions[D::Down as usize].0,
                                beem_position.1 + directions[D::Down as usize].1,
                                D::Down,
                            )],
                            D::Down => vec![(
                                beem_position.0 + directions[D::Left as usize].0,
                                beem_position.1 + directions[D::Left as usize].1,
                                D::Left,
                            )],
                            D::Up => vec![(
                                beem_position.0 + directions[D::Right as usize].0,
                                beem_position.1 + directions[D::Right as usize].1,
                                D::Right,
                            )],
                        },
                        b'-' => match beem_position.2 {
                            D::Right => vec![(
                                beem_position.0 + directions[D::Right as usize].0,
                                beem_position.1 + directions[D::Right as usize].1,
                                D::Right,
                            )],
                            D::Left => vec![(
                                beem_position.0 + directions[D::Left as usize].0,
                                beem_position.1 + directions[D::Left as usize].1,
                                D::Left,
                            )],
                            D::Down | D::Up => vec![
                                (
                                    beem_position.0 + directions[D::Left as usize].0,
                                    beem_position.1 + directions[D::Left as usize].1,
                                    D::Left,
                                ),
                                (
                                    beem_position.0 + directions[D::Right as usize].0,
                                    beem_position.1 + directions[D::Right as usize].1,
                                    D::Right,
                                ),
                            ],
                        },
                        b'|' => match beem_position.2 {
                            D::Right | D::Left => vec![
                                (
                                    beem_position.0 + directions[D::Up as usize].0,
                                    beem_position.1 + directions[D::Up as usize].1,
                                    D::Up,
                                ),
                                (
                                    beem_position.0 + directions[D::Down as usize].0,
                                    beem_position.1 + directions[D::Down as usize].1,
                                    D::Down,
                                ),
                            ],
                            D::Down => vec![(
                                beem_position.0 + directions[D::Down as usize].0,
                                beem_position.1 + directions[D::Down as usize].1,
                                D::Down,
                            )],
                            D::Up => vec![(
                                beem_position.0 + directions[D::Up as usize].0,
                                beem_position.1 + directions[D::Up as usize].1,
                                D::Up,
                            )],
                        },
                        b'.' => match beem_position.2 {
                            D::Right => vec![(
                                beem_position.0 + directions[D::Right as usize].0,
                                beem_position.1 + directions[D::Right as usize].1,
                                D::Right,
                            )],
                            D::Left => vec![(
                                beem_position.0 + directions[D::Left as usize].0,
                                beem_position.1 + directions[D::Left as usize].1,
                                D::Left,
                            )],
                            D::Down => vec![(
                                beem_position.0 + directions[D::Down as usize].0,
                                beem_position.1 + directions[D::Down as usize].1,
                                D::Down,
                            )],
                            D::Up => vec![(
                                beem_position.0 + directions[D::Up as usize].0,
                                beem_position.1 + directions[D::Up as usize].1,
                                D::Up,
                            )],
                        },

                        _ => unreachable!(),
                    };

                new_beems.into_iter().for_each(|beem| {
                    if beem.0 >= 0
                        && beem.0 < width as i32
                        && beem.1 >= 0
                        && beem.1 < height as i32
                        && !explored_map.contains(&beem)
                    {
                        explored_map.insert(beem);
                        energized_map.insert((beem.0, beem.1));
                        beem_positions.push(beem);
                    }
                });
            }
            // println!();
            // map.chunks(width).enumerate().for_each(|(y, row)| {
            //     println!("{:?}", row.iter().enumerate().map(|(x, c)| {
            //         if energized_map.contains(&(x as i32,y as i32)) { '#'} else { '.'}
            //     }).join(""));
            // });
            energized_map.len()
        })
        .max()
        .unwrap();

    println!("max: {:?}", result)
}
