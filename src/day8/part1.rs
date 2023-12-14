use crate::utils;
use std::collections::HashMap;

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);

    let mut directions: Vec<usize> = vec![];
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(row) = _line {
                if _i == 0 {
                    directions = row
                        .chars()
                        .map(|f| if f == 'L' { 0 as usize } else { 1 as usize })
                        .collect::<Vec<usize>>();
                } else if _i > 1 {
                    let key_val: Vec<String> = row.split(" = ").map(String::from).collect();
                    let _key: String = key_val[0].to_string();
                    let val_str: String = key_val[1].to_string();
                    
                    let _value = val_str
                        .split(", ")
                        .map(|f| f.replace("(", "").replace(")", ""))
                        .collect::<Vec<String>>();
                    map.insert(_key, _value);
                }
            }
        }
    }
    let mut current_key: String = "AAA".to_string();
    let mut _sum: i32 = 1;
    'lineLoop: loop {
        for _dir in &directions {
            let _value: &Vec<String> = map.get(&current_key).unwrap();
            current_key = _value[*_dir].to_string();
            if current_key == "ZZZ" {
                break 'lineLoop;
            }
            _sum += 1;
        } 
    }
    
    println!("Sum: {}", _sum);
}
