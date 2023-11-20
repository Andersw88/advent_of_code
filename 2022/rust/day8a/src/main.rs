
use itertools::Itertools;

fn main() {
    println!();

    let input = include_str!("input.csv");
    // let input = include_str!("example.csv");

    let width = input.as_bytes().iter().find_position(|c| **c == b'\n').unwrap().0;
    let height = input.lines().count();

    let trees = input.chars()
    .filter(|f| f.is_ascii_digit())
    .map(|f| f as i8 - b'0' as i8).collect_vec();

    let tree_rows: Vec::<&[i8]> = trees.chunks(width).collect();
    
    let mut trees_visibility = vec![false; trees.len()];
    
    tree_rows.into_iter().enumerate().for_each(|(i, row) | {
        // From left
        let _ = row.iter().enumerate().fold( -1,| current_height, (j, tree_height)| {
            if current_height >= *tree_height
            {
                current_height
            } 
            else {
                trees_visibility[i * width + j] |= true;
                *tree_height
            }
        });
        // From right
        let _ = row.iter().rev().enumerate().fold( -1,| current_height, (j, tree_height)| {
            if current_height >= *tree_height
            {
                current_height
            } 
            else {
                trees_visibility[i *width + (height - j - 1)] |= true;
                *tree_height
            }
        });
    });

    let mut trees_transposed: Vec<i8> = vec![0; width * height];
    transpose::transpose(&trees, &mut trees_transposed, width, height);
    let tree_columns: Vec::<&[i8]> = trees_transposed.chunks(width).collect();
    tree_columns.into_iter().enumerate().for_each(|(i, row) | {
        // From top
        let _ = row.iter().enumerate().fold( -1,| current_height: i8, (j, tree_height)| {
            if current_height >= *tree_height
            {
                current_height
            } 
            else {
                trees_visibility[i  + j * height] |= true;
                *tree_height
            }
        });
        // From bottom
        let _ = row.iter().rev().enumerate().fold( -1,| current_height, (j, tree_height)| {
            if current_height >= *tree_height
            {
                current_height
            } 
            else {
                trees_visibility[i + (height - j - 1) * height] |= true;
                *tree_height
            }
        });
    });

    trees_visibility.chunks(width).for_each(|f| {
        println!("{:?}", f.iter().map(|&b| if b { '#' } else { ' ' }).collect::<String>() );
    });
    println!();
    trees.chunks(width).for_each(|f| {
        println!("{:?}", f.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join(""))
    });
    // println!();
    // trees_transposed.chunks(width).for_each(|f| {
    //     println!("{:?}", f.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join(""));
    // });
    
    println!("{:?}", trees_visibility.iter().filter(|b| **b ).count());


}
