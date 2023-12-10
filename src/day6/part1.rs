use crate::utils;

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);

    let mut time = Vec::new();
    let mut distance = Vec::new();

    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(row) = _line {
                if row.starts_with("Time:") {
                    let numbers: Vec<u64> = row
                        .split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    time.extend(numbers);
                } else if row.starts_with("Distance:") {
                    let numbers: Vec<u64> = row
                        .split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    distance.extend(numbers);
                }
            }
        }
    }
    let mut product = 1;
    for i in 0..time.len() {
        let mut sum = 0;
        for j in 0..time[i] {
            if j * (time[i] - j) > distance[i] {
                sum += 1;
            }
        }

        if sum > 0 {
            product *= sum;
        }
    }
    println!("Product: {}", product);
}
