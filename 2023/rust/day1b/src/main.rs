use itertools::Itertools;

fn to_digit(substr: &str) -> Option<char> {
    match substr {
        _ if substr.starts_with("zero") => Some('0'),
        _ if substr.starts_with("one") => Some('1'),
        _ if substr.starts_with("two") => Some('2'),
        _ if substr.starts_with("three") => Some('3'),
        _ if substr.starts_with("four") => Some('4'),
        _ if substr.starts_with("five") => Some('5'),
        _ if substr.starts_with("six") => Some('6'),
        _ if substr.starts_with("seven") => Some('7'),
        _ if substr.starts_with("eight") => Some('8'),
        _ if substr.starts_with("nine") => Some('9'),
        _ => {
            let digit = substr.chars().next().unwrap();
            if digit.is_numeric() {
                Some(digit)
            } else {
                None
            }
        }
    }
}

fn main() {
    println!();

    // let rows = include_str!("example.txt").split('\n');
    let rows = include_str!("input.txt").split('\n');

    let result: i32 = rows
        .map(|row| {
            [
                (0..row.len())
                    .find_map(|start_index| {
                        let substr = &row[start_index..];
                        to_digit(substr)
                    })
                    .unwrap(),
                (0..row.len())
                    .find_map(|end_index| {
                        let substr = &row[row.len() - end_index - 1..row.len()];
                        to_digit(substr)
                    })
                    .unwrap(),
            ]
            .iter()
            .join("")
            .parse::<i32>()
            .unwrap()
        })
        .sum();

    println!("{:?}", result);
}
