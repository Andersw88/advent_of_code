use std::cmp::{max, min};

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum D {
    Right = 0,
    Left,
    Down,
    Up,
}

fn main() {
    println!();

    // let input = include_str!("input.txt").lines();
    let input = include_str!("example.txt").lines();

    let mut instructions: Vec<(D, i32)> = vec![];

    // input.for_each(|line| {
    //     let (_, rest) = line.split_once(' ').unwrap();
    //     let (_, hex1) = rest.split_once(' ').unwrap();
    //     let hex2 = i32::from_str_radix(&hex1[2..hex1.len()-2], 16).unwrap();
    //     let direction2 = &hex1[hex1.len()-2..hex1.len()-1].parse::<i32>().unwrap();
    //     println!("{:?}, {:?}", hex2, direction2);
    //     match direction2 {
    //         1 => instructions.push((D::Down, hex2)),
    //         3 => instructions.push((D::Up, hex2)),
    //         2 => instructions.push((D::Left, hex2)),
    //         0 => {
    //             instructions.push((D::Right, hex2))
    //         }
    //         _ => unreachable!(),
    //     }
    // });

    input.for_each(|line| {
        let (c, rest) = line.split_once(' ').unwrap();
        let (num, _) = rest.split_once(' ').unwrap();
        match c {
            _ if line.starts_with('D') => instructions.push((D::Down, num.parse::<i32>().unwrap())),
            _ if line.starts_with('U') => instructions.push((D::Up, num.parse::<i32>().unwrap())),
            _ if line.starts_with('L') => instructions.push((D::Left, num.parse::<i32>().unwrap())),
            _ if line.starts_with('R') => {
                instructions.push((D::Right, num.parse::<i32>().unwrap()))
            }
            _ => unreachable!(),
        }
    });

    instructions.chunks(2).into_iter().map(|b| {
        let b1 = b[0];
        let b2 = b[1];
    });



    println!("instructions: {:?}", instructions);
    println!("instructions: {:?}", instructions.iter().map(|(dir, _)| if *dir == D::Up || *dir == D::Down { "#" } else { "." }).collect_vec());


}
