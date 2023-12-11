use itertools::Itertools;

fn main() {
    println!();

    // let input = include_str!("example.txt").lines();
    let input = include_str!("input.txt").lines();

    let number_lines: Vec<Vec<i32>> = input
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let result = number_lines
        .iter()
        .map(|line| {
            let mut diff_line: Vec<Vec<i32>> = vec![line.clone(); 1];

            loop {
                let curr_line = diff_line.last().unwrap();
                let next_line = curr_line
                    .windows(2)
                    .map(|values| values[1] - values[0])
                    .collect_vec();

                if next_line.iter().all(|c| *c == 0) {
                    diff_line.push(next_line);
                    break;
                }
                diff_line.push(next_line);
            }
            // println!("{:?}", diff_line);

            let (_, sum) = diff_line
                .iter()
                .map(|line| line.last().unwrap())
                .fold((0, 0), |(prev, sum), value| {
                    // println!("|{:?}", (prev + value, prev + value + sum));
                    (prev + value, value + sum)
                });
            sum
        })
        .sum::<i32>();
    println!("{:?}", result);
}
