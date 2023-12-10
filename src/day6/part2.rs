use crate::utils;

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);

    let mut time_num: u64 = 0;
    let mut distance_num: u64 = 0;

    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(row) = _line {
                if row.starts_with("Time:") {
                    let numbers: Vec<u64> = row
                        .split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    time_num = numbers
                        .into_iter()
                        .map(|i| i.to_string())
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap();
                } else if row.starts_with("Distance:") {
                    let numbers: Vec<u64> = row
                        .split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    distance_num = numbers
                        .into_iter()
                        .map(|i| i.to_string())
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap();
                }
            }
        }
    }

    let mut sum = 0;
    for j in 0..time_num {
        if j * (time_num - j) > distance_num {
            sum += 1;
        }
    }
    println!("Sum: {}", sum)
}
