
fn main() {
    println!();

    let input = include_str!("input.csv").split('\n');
    // let input = include_str!("example.csv").split('\n');
    let mut x = 1;
    let mut cycle = 0;

    let width = 40;
    let height = 6;
    let mut screen = vec![false; width * height];
    // let mut screen2 = vec![0; width * height];
    input.for_each(|row| {
        match row
        {
            _ if row == "noop" => {
                screen[cycle as usize] = (cycle % width as i32).abs_diff(x) <= 1;
                // screen2[cycle as usize] = (cycle % width as i32).abs_diff(x) as i32;
            },
            _ if row.starts_with("addx") => {
                screen[cycle as usize] = (cycle % width as i32).abs_diff(x) <= 1;
                // screen2[cycle as usize] = (cycle % width as i32).abs_diff(x)  as i32;
                cycle += 1;
                screen[cycle as usize] = (cycle % width as i32).abs_diff(x) <= 1;
                // screen2[cycle as usize] = (cycle % width as i32).abs_diff(x) as i32;
                x += row.split_once(' ').unwrap().1.parse::<i32>().unwrap();
            },
            _ => panic!()
        }
        cycle += 1;

    });

    screen.chunks(width).for_each(|row| {
        println!("{:?}", row.iter().map(|&num| (if num { '#' } else {'.'})).collect::<String>());
    });
    // screen2.chunks(width).for_each(|row| {
    //     println!("{:?}", row.iter().map(|&num| (num).to_string()).collect::<Vec<String>>().join(" "));
    // });

}
