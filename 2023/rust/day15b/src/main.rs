use indexmap::IndexMap;

fn main() {
    println!();

    let input = include_str!("input.txt").lines();
    // let input = include_str!("example.txt").lines();

    let mut boxes = vec![IndexMap::<String, i32>::new(); 256];

    input.for_each(|line| {
        line.split_terminator(',').for_each(|s| {
            let boxnr = s
                .as_bytes()
                .iter()
                .take_while(|&&b| b != b'=' && b != b'-')
                .fold(0_i32, |current_value, &sequence| {
                    (current_value + sequence as i32) * 17 % 256
                });

            if let Some((label, focal_length)) = s.split_once('=') {
                boxes[boxnr as usize]
                    .insert(label.to_string(), focal_length.parse::<i32>().unwrap());
            } else if let Some(label) = s.strip_suffix('-') {
                boxes[boxnr as usize].shift_remove(label);
            }
        });
    });

    let result = boxes
        .iter()
        .enumerate()
        .map(|(i, boxx)| {
            boxx.iter()
                .enumerate()
                .map(|(j, (_, &value))| (i + 1) * (j + 1) * value as usize)
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{:?}", result);
}
