use std::env;
mod utils;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let mut _sum: i32 = 0;
    if let Ok(lines) = utils::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                let mut digits = vec![];
                for r in row.chars() {
                    if r.is_digit(10) {
                        digits.push((r as i32) - 0x30);
                    }
                }
                if digits.len() > 0 {
                    _sum += digits[0] * 10 + digits[digits.len() - 1];
                }
            }
        }
    }
    println!("{}", _sum);
}

