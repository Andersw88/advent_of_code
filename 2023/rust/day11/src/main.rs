
use itertools::Itertools;

fn main() {
    println!();

    // let input = include_str!("example.txt").lines();
    let input = include_str!("input.txt").lines();

    let height = input.clone().count();
    let width = input.clone().next().unwrap().len();

    let mut occupied_colums = vec![0; width];
    let mut occupied_rows = vec![0; height];

    let mut galaxies = vec![];
    input.enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                galaxies.push((x, y));
                occupied_colums[x] = 1;
                occupied_rows[y] = 1;
            }
        })
    });

    // let expansion = 2 - 1; // part 1
    let expansion = 1000000 - 1; // part 2

    let mut galaxies2 = vec![];
    occupied_colums
        .iter()
        .enumerate()
        .fold(0, |mut offset, (i, &occupied)| {
            if occupied == 0 {
                offset += expansion;
            } else {
                galaxies
                    .iter()
                    .filter(|(x, _)| *x == i)
                    .for_each(|(x, y)| galaxies2.push((x + offset, *y)));
            }
            offset
        });

    let mut galaxies3 = vec![];
    occupied_rows
        .iter()
        .enumerate()
        .fold(0, |mut offset, (i, &occupied)| {
            if occupied == 0 {
                offset += expansion;
            } else {
                galaxies2
                    .iter()
                    .filter(|(_, y)| *y == i)
                    .for_each(|(x, y)| galaxies3.push((*x, y + offset)));
            }
            offset
        });

    // println!("{:?}", galaxies);
    // println!("{:?}", galaxies2);
    // println!("{:?}", galaxies3);
    
    let distances_sum: usize =  galaxies3.iter().combinations(2).map(|comb| {
        let (x1, y1) = comb[0];
        let (x2, y2) = comb[1];
        
        x1.abs_diff(*x2) + y1.abs_diff(*y2)
    }).sum();
    println!("{:?}", distances_sum);
}
