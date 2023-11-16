use itertools::Itertools;

fn main() {
    println!();

    let rows = include_str!("input.csv");
    let marker_length = 14;
    let column = rows.as_bytes().windows(marker_length).enumerate().find_map(|(i, c)|
    {    
        let endmarker_found = c.iter().unique().count() == marker_length;
        endmarker_found.then_some(i + marker_length)
    });

    print!("{:?}", column.unwrap());

}
