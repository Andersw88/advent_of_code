use std::collections::HashMap;

struct Game {
    id: u32,
    cubes: Vec<HashMap<String, u32>>,
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");

            let id = parts
                .next()
                .unwrap()
                .trim()
                .trim_start_matches("Game ")
                .parse()
                .unwrap();
            let cubes_data: Vec<&str> = parts.next().unwrap().split("; ").collect();

            let cubes: Vec<HashMap<String, u32>> = cubes_data
                .iter()
                .map(|subset| {
                    let mut cube_map: HashMap<String, u32> = HashMap::new();
                    let subset_parts: Vec<&str> = subset.split(", ").collect();
                    for subset_part in subset_parts {
                        let cube: Vec<&str> = subset_part.split_whitespace().collect();
                        let count: u32 = cube[0].parse().unwrap();
                        let color = cube[1].to_string();
                        cube_map.insert(color, count);
                    }
                    cube_map
                })
                .collect();

            Game { id, cubes }
        })
        .collect()
}

fn main() {
    println!();

    // let input = include_str!("example.txt");
    let input = include_str!("input.txt");

    let max_cubes: HashMap<String, u32> = [
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]
    .iter()
    .cloned()
    .collect();

    let games = parse_input(input);

    let mut game_sum = 0;
    for game in games {
        let is_game_inpossible = game
            .cubes
            .iter()
            .flatten()
            .map(|(key, cube_count)| cube_count > max_cubes.get(key).unwrap())
            .any(|f| f);

        if !is_game_inpossible {
            game_sum += game.id;
        }

        println!("Game {}: {:?}", game.id, game.cubes);
    }
    println!("Game sum {}", game_sum);
}
