use itertools::Itertools;

fn main() {
    println!();

    let input = include_str!("example3.txt").lines();
    // let input = include_str!("input.txt").lines();

    let mut start = (0, 0);

    let pipe_map = input.clone()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '|' => [0_usize, 0_usize, 1_usize, 1_usize],
                    '-' => [1, 1, 0, 0],
                    'L' => [0, 1, 1, 0],
                    'J' => [1, 0, 1, 0],
                    '7' => [1, 0, 0, 1],
                    'F' => [0, 1, 0, 1],
                    'S' => {
                        start = (j, i);
                        [1, 1, 1, 1]
                    }
                    '.' => [0, 0, 0, 0],
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let height = input.count();
    let width = pipe_map.len() / height;
    let mut expanded_map = vec![0; width * height];
    let mut open_list: Vec<(usize, usize, usize)> = [(start.0, start.1, 0)].into();

    let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while let Some(&next) = open_list.first() {
        open_list.remove(0);
        let pipe_next = pipe_map[next.0 + next.1 * width];

        expanded_map[next.0 + next.1 * width] = next.2 + 1;

        for (i, _) in pipe_next.iter().enumerate().filter(|(_, open)| {
            **open == 1
        }) {
            let direction = directions[i];
            let (x, y) = (next.0 as i32 + direction.0, next.1 as i32 + direction.1);

            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                let other_pipe = pipe_map[x as usize + y as usize * width];

                let other_direction_i = match i {
                    0 => 1,
                    1 => 0,
                    2 => 3,
                    3 => 2,
                    _ => unreachable!(),
                };

                if other_pipe[other_direction_i] == 1 && pipe_next[i] == 1 {
                    let expanded_step = &mut expanded_map[x as usize + y as usize * width];
                    
                    if *expanded_step == 0
                    {
                        open_list.push((x as usize, y as usize, next.2 + 1));
                    }
                }
            }
        }
    }

    // for line in expanded_map.chunks(width)
    // {
    //     println!("{:?}", line.iter().map(|c| *c).join(" "));
    // }

    let result = expanded_map.iter().max().unwrap() - 1;
    println!("{:?}", result);
}
