fn main() {
    println!();

    let cals = include_str!("input.csv")
        .split('\n')
        .map(|turn| match turn {
            "A X" => 1 + 3, // Rock - Rock
            "B X" => 1,     // Paper - Rock
            "C X" => 1 + 6, // Scissors - Rock
            "A Y" => 2 + 6, // Rock - Paper
            "B Y" => 2 + 3, // Paper - Paper
            "C Y" => 2,     // Scissors - Paper
            "A Z" => 3,     // Rock - Scissors
            "B Z" => 3 + 6, // Paper - Scissors
            "C Z" => 3 + 3, // Scissors - Scissors
            _ => panic!(),
        })
        .sum::<i32>();

    println!("{:?}", cals);
}
