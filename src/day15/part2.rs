use crate::utils;

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {}
        }
    }
}
