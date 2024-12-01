fn main() {
    println!();

    let input = include_str!("input.txt").lines();
    // let input = include_str!("example.txt").lines();

    let mut instructions: Vec<(i64, i64)> = vec![];

    instructions.push((0, 0));

    input.for_each(|line| {
        let (_, rest) = line.split_once(' ').unwrap();
        let (_, hex) = rest.split_once(' ').unwrap();

        let length = i64::from_str_radix(&hex[2..=6], 16).unwrap();
        let mut current_pos = *instructions.last().unwrap();
        current_pos = match hex.chars().nth_back(1).unwrap() {
            '1' => (current_pos.0, current_pos.1 + length),
            '3' => (current_pos.0, current_pos.1 - length),
            '2' => (current_pos.0 - length, current_pos.1),
            '0' => (current_pos.0 + length, current_pos.1),
            _ => unreachable!(),
        };
        instructions.push(current_pos)
    });

    let result = instructions.windows(2).fold(0, |area, b| {
        area + (b[0].0) * (b[1].1) - (b[0].1) * (b[1].0)
            + (i64::abs(b[0].0 - b[1].0) + i64::abs(b[0].1 - b[1].1))
    }) / 2;

    println!("area: {:?}", result + 1);
}
