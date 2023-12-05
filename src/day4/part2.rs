use crate::utils;

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);

    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (i, line) in lines.enumerate() {
        }
    }
}
