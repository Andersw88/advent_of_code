fn main() {
    println!();

    let cals = include_str!("input.csv")
        .split('\n')
        .map(|turn| match turn {
            "A X" => 3,     // Rock - Scissors
            "B X" => 1,     // Paper - Rock
            "C X" => 2,     // Scissors - Paper
            "A Y" => 1 + 3, // Rock - Rock
            "B Y" => 2 + 3, // Paper - Paper
            "C Y" => 3 + 3, // Scissors - Scissors
            "A Z" => 2 + 6, // Rock - Paper
            "B Z" => 3 + 6, // Paper - Scissors
            "C Z" => 1 + 6, // Scissors - Rock
            _ => panic!(),
        })
        .sum::<i32>();

    println!("{:?}", cals);
}
