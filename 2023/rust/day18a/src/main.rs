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

    let input = include_str!("input.txt").lines();
    // let input = include_str!("example.txt").lines();

    let mut instructions: Vec<(D, i32)> = vec![];

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

    let mut x_max: i32 = 0;
    let mut y_max: i32 = 0;
    let mut x_min = i32::MAX;
    let mut y_min = i32::MAX;
    instructions.iter().fold((0, 0), |(x, y), (dir, len)|{
        let (mut nx, mut ny) = match dir {
            D::Right => (x + len, y),
            D::Left => (x - len, y),
            D::Down => (x, y + len),
            D::Up => (x, y - len),
        };

        x_max = max(x_max, nx );
        y_max = max(y_max, ny );
        x_min = min(x_min, nx );
        y_min = min(y_min, ny );
        (nx, ny)
    });

    let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let width: usize = (x_max - x_min) as usize + 1;
    let height: usize = (y_max - y_min) as usize + 1;

    let mut map = vec![0_u8; width * height];
    instructions.iter().fold((-x_min, -y_min), |(mut x, mut y), (dir, len)|
    {   
        (0..*len).for_each(|_| {
            x += directions[*dir as usize].0;
            y += directions[*dir as usize].1;

            map[x as usize + y as usize * width] = 1;
        });

        (x,y)
    });

    println!("instructions: {:?}", instructions);

    map.chunks(width).for_each(|row| {
        println!("{:?}", row.iter().map(|&num| (if num == 1 { '#' } else {'.'})).collect::<String>());
    });

    flood_fill(&mut map, width as i32 / 2,height as i32/ 2, width, height);

    println!();
    map.chunks(width).for_each(|row| {
        println!("{:?}", row.iter().map(|&num| (if num == 1 { '#' } else {'.'})).collect::<String>());
    });

    let result = map.iter().filter(|b| **b == 1).count();

    println!("Result: {:?}", result);

}


fn flood_fill(map: &mut [u8], x: i32, y: i32, width: usize, height: usize) {
    let mut to_fill = vec![(x, y)];
    let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let Some((x, y)) = to_fill.pop() {
        if x >= 0
            && x < width as i32
            && y >= 0
            && y < height as i32
            && map[(y * width as i32 + x) as usize] != 0
        {
            continue;
        }
        map[(x + y * width as i32) as usize] = 1;

        for direction in &directions {
            let x2 = x + direction.0;
            let y2 = y + direction.1;

            to_fill.push((x2, y2));
        }
    }
}