use crate::utils;

fn get_hash_string(hash_string: String, hash_init: i32) -> i32 {
    let mut hash_val = hash_init;
    for ch in hash_string.chars() {
        let step = ((hash_val + (ch as u32) as i32) * 17) as i32;
        hash_val = step % 256;
    }
    return hash_val;
}


pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut _sum: i32 = 0;
    
    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                let mut hash_strings = _row.split(",").collect::<Vec<&str>>();
                for hash_string in hash_strings.iter_mut() {
                    *hash_string = hash_string.trim();
                    _sum += get_hash_string(hash_string.to_string(), 0);
                }
            }
        }
    }
    println!("Sum: {}", _sum);
}
