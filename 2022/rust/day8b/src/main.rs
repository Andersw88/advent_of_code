
use itertools::Itertools;

fn main() {
    println!();

    let input = include_str!("input.csv");
    // let input = include_str!("example.csv");

    let width = input.as_bytes().iter().find_position(|c| **c == b'\n').unwrap().0;
    let height = input.lines().count();

    let trees = input.chars()
        .filter(|f| f.is_ascii_digit())
        .map(|f| f as i32 - b'0' as i32).collect_vec();

    let mut trees_visibility = vec![1; trees.len()];


    for (x, y) in (1..width - 1).cartesian_product(1..height - 1) {
        let current_height = trees[x + y * width];
        for n in (0..x).rev()
        {
            if trees[n + y * width] >= current_height || n == 0
            {
                trees_visibility[x + y * width] *= x - n;
                break;
            }
        }
        for n in x + 1..width
        {
            if trees[n + y * width] >= current_height || n == width - 1
            {
                trees_visibility[x + y * width] *= n - x;
                break;
            }
        }
        for n in (0..y).rev()
        {
            if trees[x + n * width] >= current_height || n == 0
            {
                trees_visibility[x + y * width] *= y - n;
                break;
            }
        }
        for n in y + 1..height
        {
            if trees[x + n * width] >= current_height || n == height - 1
            {
                trees_visibility[x + y * width] *= n - y;
                break;
            }
        }
    }

    trees_visibility.chunks(width).for_each(|f| {
        println!("{:?}", f.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join(" "));
    });
    trees.chunks(width).for_each(|f| {
        println!("{:?}", f.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join(""))
    });

    println!("{:?}", trees_visibility.iter().max());


}
