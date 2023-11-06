use itertools::enumerate;

fn main() {
    println!();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_path("2022/rust/day1/src/input.csv")
        .expect("Should have been able to read the file");

    let mut rows: Vec<Vec<i32>> = vec![vec![]];

    for result in rdr.records() {
        let record: csv::StringRecord = result.unwrap();
        for field in &record {
            if let Ok(value) = field.to_owned().parse::<i32>() {
                let row = rows.last_mut().unwrap();
                row.push(value);
            } else {
                rows.push(Vec::new());
            }
        }
    }

    let mut sums = vec![0; rows.len()];
    for (i, row) in enumerate(rows.iter()) {
        sums[i] = row.iter().sum();
    }
    sums.sort();

    println!("{:?}", sums[sums.len() - 3..].iter().sum::<i32>());
}
