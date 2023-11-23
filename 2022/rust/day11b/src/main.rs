use std::mem;

use itertools::Itertools;

struct Monkey {
    items: Vec<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    divisable: i64,
    target_ids: (usize, usize),
    inspects: i64 ,
}

fn main() {
    println!();

    let input = include_str!("input.csv").split('\n');
    // let input = include_str!("example.csv").split('\n');

    let mut monkey_vec = input.chunks(7).into_iter().map(|mut monkey_lines| {
        monkey_lines.next(); //Id

        let items = monkey_lines.next().unwrap()["  Starting items: ".len()..]
            .split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect_vec();
        let operation = monkey_lines.next().unwrap()["  Operation: new = old ".len()..]
            .split_once(' ')
            .map(|(operator, i2)| {
                if operator == "*" {
                    if let Ok(item2) = i2.parse::<i64>() {
                        Box::new(move |old| old * item2)
                    } else {
                        Box::new(|old| {
                            old * old
                        }) as Box<dyn Fn(i64) -> i64>
                    }
                } else if let Ok(item2) = i2.parse::<i64>() {
                    Box::new(move |old| old + item2)
                } else {
                    Box::new(|old| old + old) as Box<dyn Fn(i64) -> i64>
                }
            }).unwrap();
        let divisable = monkey_lines.next().unwrap()["  Test: divisible by ".len()..].parse::<i64>().unwrap();
        let target_ids = (monkey_lines.next().unwrap()["    If true: throw to monkey ".len()..].parse::<usize>().unwrap(),
                                      monkey_lines.next().unwrap()["    If false: throw to monkey ".len()..].parse::<usize>().unwrap());
        

        Monkey {
            items,
            operation,
            divisable,
            target_ids,
            inspects:0
        }
    }).collect_vec();

    let sum_divisors: i64 = monkey_vec.iter().map(|f|f.divisable).product();
    let num_monkeys = monkey_vec.len();
    for n in 0..num_monkeys*10000 {
        let monkey = &mut monkey_vec[n % num_monkeys];

        let mut target1_vec: Vec<_> = vec![];
        let mut target2_vec: Vec<_> = vec![];
        for worry_level in mem::take(&mut monkey.items)
        {   
            let worry_level = (monkey.operation)(worry_level) % sum_divisors;

            if worry_level % monkey.divisable == 0
            {
                target1_vec.push(worry_level);
            }
            else
            {
                target2_vec.push(worry_level)
            }
            monkey.inspects += 1;
        }
        let targets = monkey.target_ids;
        monkey_vec[targets.0].items.append(&mut target1_vec);
        monkey_vec[targets.1].items.append(&mut target2_vec);
    }

    let result = monkey_vec.iter().map(|f| f.inspects).sorted().rev().take(2).product::<i64>();
    println!("{:?}", result);
}
