use itertools::Itertools;

fn main() {
    println!();

    let rows = include_str!("input.csv");
    let marker_length = 4;
    let column = rows.as_bytes().windows(marker_length).enumerate().find_map(|(i, c)|
    {    
        if c.iter().unique().count() == marker_length
        {
            Some(i + marker_length)
        }
        else
        {
            None
        }
    });

    print!("{:?}", column.unwrap());

}
