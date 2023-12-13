use itertools::Itertools;

fn main() {
    println!();

    let input = include_str!("example.txt").lines();
    // let input = include_str!("input.txt").lines();

    let result: usize = input
        .map(|row| {
            let (springs, spring_groups) = row.split_once(' ').unwrap();
            let row: Vec<i32> = springs
                .chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    '?' => 2,
                    _ => unreachable!(),
                })
                .collect_vec();
            let spring_spans = spring_groups
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect_vec();

            let successful_permutations = (0_usize..row.len())
                .permutations(spring_spans.len())
                .filter(|positions| {
                    for positions2 in positions.windows(2) {
                        if positions2[0] > positions2[1] {
                            return false;
                        }
                    }

                    let mut row2 = row.clone();
                    for (span_index, &position) in positions.iter().enumerate() {
                        let span_length = spring_spans[span_index];

                        if position + span_length > row2.len() {
                            return false;
                        }

                        if position > 0
                            && !(row2[position - 1] == 0
                                || row2[position - 1] == 2)
                        {
                            return false;
                        }
                        if (position + span_length) < row2.len()
                            && !(row2[position + span_length] == 0
                                || row2[position + span_length] == 2)
                        {
                            return false;
                        }

                        for spring in &mut row2[position..position + span_length]
                        {
                            if *spring == 0 || *spring == 3 {
                                return false;
                            } else {
                                *spring = 3
                            }
                        }

                        if position > 0 {
                            row2[position - 1] = 0;
                        }
                        if (position + span_length) < row2.len() {
                            row2[position + span_length] = 0;
                        }
                    }
                    if row2.iter().any(|&c| c == 1)
                    {
                        return false
                    }
                    println!("positions {:?},{:?}", row2, positions);
                    true
                })
                .count();

            println!("successful_permutations {:?},", successful_permutations);
            successful_permutations

        })
        .sum();

    println!("sum {:?}", result);
}
