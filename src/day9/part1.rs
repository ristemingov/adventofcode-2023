use crate::utils;

fn extrapolation_backtrack(numbers: Vec<i64>) -> i64 {
    if numbers.clone().into_iter().all(|b| b == 0) {
        return 0;
    }
    let mut diferences: Vec<i64> = Vec::new();
    for i in 1..numbers.len() {
        diferences.push(numbers[i] - numbers[i - 1]);
    }
    return numbers[numbers.len() - 1] + extrapolation_backtrack(diferences);
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut _sum: i64 = 0;
    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                let numbers: Vec<i64> = _row
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                let result: i64 = extrapolation_backtrack(numbers);
                _sum += result;
            }
        }
    }
    println!("{}", _sum);
}
