use cached::proc_macro::cached;
use cached::{Cached, UnboundCache};
use itertools::Itertools;

#[cached(
    type = "UnboundCache<(usize,usize), usize>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ (j,depth) }"#,
    name = "RANGES_CACHE"
)]
fn ranges(
    j: usize,
    depth: usize,
    row: &Vec<i32>,
    spring_spans: &Vec<usize>,
) -> usize {
    let curr_span_index = spring_spans.len() - depth;
    let len = row.len();

    let last_viable_start = row[j..len]
        .iter()
        .enumerate()
        .find_map(|(i, &c)| if c == 1 { Some(i + j) } else { None })
        .unwrap_or(len);

    let remaining_space_needed: usize = spring_spans[(curr_span_index + 1)..spring_spans.len()]
        .iter()
        .map(|c| c + 1)
        .sum::<usize>();

    let end_range = std::cmp::min(last_viable_start, len - remaining_space_needed - 1);

    let mut permutations: usize = 0;
    for i in j..=end_range {

        let curr_span_end = spring_spans[curr_span_index] + i;
        if curr_span_end < row.len() && row[curr_span_end] == 1 {
            continue;
        }

        if curr_span_end > row.len() || row[i..curr_span_end].iter().any(|&c| c == 0) {
            continue;
        }

        if depth == 1 {
            if row[curr_span_end..len].iter().any(|&c| c == 1) {
                continue;
            }
            permutations += 1;
            continue;
        }

        if curr_span_end + 1 > len {
            continue;
        }

        permutations += ranges(
            curr_span_end + 1, // One free space between
            depth - 1,
            row,
            spring_spans,
        );
    }
    return permutations;
}

fn main() {
    println!();

    // let input = include_str!("example.txt").lines();
    let input = include_str!("input.txt").lines();
    let cycles = 5;

    let permuations = input
        .clone()
        .map(|row| {
            let (springs, spring_groups) = row.split_once(' ').unwrap();
            let mut row2: Vec<i32> = springs
                .chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    '?' => 2,
                    _ => unreachable!(),
                })
                .collect_vec();
            row2.push(2);
            let mut row: Vec<i32> = vec![row2; cycles]
                .iter()
                .flat_map(|n| n.clone())
                .collect_vec();
            row.pop();

            let spring_spans: Vec<usize> = std::iter::repeat(
                spring_groups
                    .split(',')
                    .map(|c| c.parse::<usize>().unwrap()),
            )
            .take(cycles)
            .flatten()
            .collect_vec();

            // println!("{:?},{:?}", row, spring_spans);
            RANGES_CACHE.lock().unwrap().cache_clear();
            ranges(
                0,
                spring_spans.len(),
                &row,
                &spring_spans,
            )
        })
        .collect_vec();

    println!("permuations {:?}", permuations);
    println!("result {:?}", permuations.iter().sum::<usize>());
}
