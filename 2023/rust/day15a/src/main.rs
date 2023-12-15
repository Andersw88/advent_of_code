fn main() {
    println!();

    let input = include_str!("input.txt").lines();
    // let input = include_str!("example.txt").lines();

    let result = input
        .map(|line| {
            line.split_terminator(',')
                .map(|s| {
                    s.as_bytes().iter().fold(0_i32, |current_value, &sequence| {
                        (current_value + sequence as i32) * 17 % 256
                    })
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("{:?}", result);
}
