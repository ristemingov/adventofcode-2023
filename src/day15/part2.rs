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
    let mut boxes: Vec<Vec<String>> = vec![[].to_vec();256];

    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                let mut hash_strings = _row.split(",").collect::<Vec<&str>>();
                for hash_string in hash_strings.iter_mut() {
                    *hash_string = hash_string.trim();
                    let mut pat: &str = "=";
                    if hash_string.contains("-") {
                        pat = "-";
                    }
                    let new_hash_string = hash_string.split(pat).collect::<Vec<&str>>();
                    let hash_val = get_hash_string(new_hash_string[0].to_string(), 0);
                    let mut found = -1;
                    for (i,st) in boxes[hash_val as usize].iter().enumerate() {
                        if st.contains(new_hash_string[0]) {
                            found = i as i32;
                            break;
                        }
                    }
                    if found >= 0 {
                        if pat == "-" {
                            boxes[hash_val as usize].remove(found as usize);
                        } else {
                            boxes[hash_val as usize][found as usize] = format!("{} {}", new_hash_string[0].to_string(), new_hash_string[1].to_string());
                        }
                    } else {
                        if pat == "=" {
                            boxes[hash_val as usize].push(format!("{} {}", new_hash_string[0].to_string(), new_hash_string[1].to_string()));
                        }
                    }
                }
            }
        }
    }
    
    for i in 0..boxes.len() {
        for j in 0..boxes[i].len() {
            _sum += (i+1) as i32 * (j+1) as i32 * (boxes[i][j].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap()) as i32;
        }
    }

    println!("Sum: {}", _sum);
}
