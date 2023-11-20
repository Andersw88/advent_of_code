use std::collections::HashSet;

fn main() {
    println!();

    let input = include_str!("input.csv").split('\n');
    // let input = include_str!("example.csv").split('\n');

    let mut hash_set: HashSet<(i32, i32)> = HashSet::new();
    let mut tails = [(0, 0); 10];

    input
        .filter_map(|f| f.split_once(' '))
        .filter_map(|(direction_str, steps_str)| {
            steps_str.parse::<i32>().ok().and_then(|steps| {
                match direction_str {
                    "R" => Some((1, 0)),
                    "L" => Some((-1, 0)),
                    "U" => Some((0, 1)),
                    "D" => Some((0, -1)),
                    _ => None,
                }
                .map(|direction| (direction, steps))
            })
        })
        .for_each(|(direction, steps)| {
            for _ in 0..steps {
                let tails_len = tails.len() - 1;
                for i in 0..tails_len {
                    if i == 0 {
                        let head = &mut tails[i];
                        *head = (head.0 + direction.0, head.1 + direction.1);
                    }
                    let head = tails[i];
                    let tail = &mut tails[i + 1];
                    if i32::abs(head.0 - tail.0) > 1 || i32::abs(head.1 - tail.1) > 1 {
                        *tail = (
                            tail.0 + i32::signum(head.0 - tail.0),
                            tail.1 + i32::signum(head.1 - tail.1),
                        );
                    }
                    if i == tails_len - 1 {
                        hash_set.insert(*tail);
                    }
                }
            }
        });

        let (max_x, min_x, max_y, min_y) = hash_set.iter().chain(tails.iter()).fold(
            (i32::MIN, i32::MAX, i32::MIN, i32::MAX),
            |(max_x, min_x, max_y, min_y), value| {
                (
                    if max_x > value.0 { max_x } else { value.0 },
                    if min_x < value.0 { min_x } else { value.0 },
                    if max_y > value.1 { max_y } else { value.1 },
                    if min_y < value.1 { min_y } else { value.1 },
                )
            },
        );
        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;
        let mut board = vec![' '; (width * height) as usize];
        hash_set.iter().for_each(|(x,y)| {
            board[(x - min_x + (y - min_y) * width) as usize] = '#';
        });
        for (i, (x,y)) in tails.iter().enumerate().rev()
        {
            board[(x - min_x + (y - min_y) * width) as usize] = i.to_string().chars().next().unwrap();
        }

        // println!("Direction:{:?}, Steps:{:?}",direction, steps);
        board.chunks(width as usize).rev().for_each(|f| {
            println!("{:?}", f.iter().map(|&num| num.to_string()).collect::<Vec<String>>().join(""));
        });
        println!();

    println!("{:?}", hash_set.len());
}
