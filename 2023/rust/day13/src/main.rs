use itertools::Itertools;

fn test_mirror(mirror: &Vec<Vec<i32>>, defects: usize) -> Option<usize> {
    let mirror_index = (1..mirror.len())
        .filter(|&i| {
            mirror[0..i]
                .iter()
                .rev()
                .zip(mirror[i..mirror.len()].iter())
                .map(|(left, right)| {
                    left.iter()
                        .zip(right.iter())
                        .filter(|(l, r)| l != r)
                        .count()
                })
                .sum::<usize>()
                == defects
        })
        .collect_vec();

    mirror_index.iter().cloned().next()
}

fn main() {
    println!();

    // let defects = 0; // Part 1
    let defects = 1; // Part 2

    // let input = include_str!("example.txt").lines();
    let input = include_str!("input.txt").lines();

    let mirrors = input
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter_map(|(is_empty, group)| {
            if is_empty {
                None
            } else {
                Some(
                    group
                        .into_iter()
                        .map(|line| {
                            line.bytes()
                                .map(|b| if b == b'#' { 1 } else { 0 })
                                .collect_vec()
                        })
                        .collect_vec(),
                )
            }
        })
        .collect_vec();

    let result = mirrors
        .iter()
        .map(|mirror| {
            if let Some(mirror_option) = test_mirror(mirror, defects) {
                return mirror_option * 100;
            }

            let mut mirror2 = vec![vec![0; mirror.len()]; mirror[0].len()];

            for (i, mirror_row) in mirror.iter().enumerate() {
                for (j, &block) in mirror_row.iter().enumerate() {
                    mirror2[j][i] = block;
                }
            }

            if let Some(mirror_option) = test_mirror(&mirror2, defects) {
                return mirror_option;
            }
            unreachable!()
        })
        .sum::<usize>();

    println!("result: {:?}", result);
}
