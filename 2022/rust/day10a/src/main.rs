use std::collections::HashSet;

fn main() {
    println!();

    let input = include_str!("input.csv").split('\n');
    // let input = include_str!("example.csv").split('\n');
    let mut x = 1;
    let mut cycle = 0;
    let x_values: i32 = 
        input.filter_map(|row| {
            let x_before = x;
            let cycle_before = cycle;
            match row
            {
                _ if row == "noop" => {
                    cycle += 1;
                },
                _ if row.starts_with("addx") => {
                    let value = row.split_once(' ').unwrap().1.parse::<i32>().unwrap();
                    x += value;
                    cycle += 2;
                },
                _ => panic!()
            }
            let cycle_devisor = (cycle + 20) / 40 - 1;
            let new_readout = (cycle_before + 20) / 40 - 1 != cycle_devisor;
            // new_readout.then_some((x_before, (cycle_devisor * 40 + 20)))
            new_readout.then_some(x_before * (cycle_devisor * 40 + 20))
        }).sum();


    println!("{:?}", (x,cycle));
    println!("{:?}", (x_values));
}
