use crate::utils;

use std::collections::HashMap;


fn sum_springs(springs: &[i32]) -> i32 {
    springs.iter().sum()
}


fn recurse(lava: &str, springs: &[i32], cache: &mut HashMap<(String, Vec<i32>), i128>) -> i128 {
    if springs.is_empty() {
        return if lava.contains('#') { 0 } else { 1 };
    }

    let current = springs[0];
    let remaining_springs = &springs[1..];
    let mut result: i128 = 0;

    for i in 0..=lava.len().saturating_sub((sum_springs(remaining_springs) + remaining_springs.len() as i32 + current) as usize) {
        if lava[..i].contains('#') {
            break;
        }
        let next = i + current as usize;
        if next <= lava.len() && !lava[i..next].contains('.') && lava.get(next..=next) != Some(&"#") {
            if lava.len() <= (next + 1) as usize {
                result += 1;
                continue;
            }
            let sub_lava = &lava[next + 1..];
            let key = (sub_lava.to_string(), remaining_springs.to_vec());
            if !cache.contains_key(&key) {
                let new_value = recurse(sub_lava, remaining_springs, cache);
                cache.insert(key.clone(), new_value);
            }
            result += cache[&key];
        }
    }

    return result
}



pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut hot_springs_map: Vec<String> = Vec::new();
    let mut hot_springs_calculations: Vec<Vec<i32>> = Vec::new();
    let mut _sum: i128 = 0;
    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                let inputs: Vec<String> = _row.split(" ").map(|s| s.to_string()).collect();
                hot_springs_map.push(inputs[0].clone());
                hot_springs_calculations.push(inputs[1].split(",").filter_map(|s| s.parse().ok()).collect());
                
                let lava = std::iter::repeat(hot_springs_map[_i].as_str()).take(5).collect::<Vec<_>>().join("?");
                let springs: Vec<i32> = hot_springs_calculations[_i].iter().cloned().cycle().take(hot_springs_calculations[_i].len() * 5).collect::<Vec<_>>();
                let mut cache = HashMap::new();
                _sum += recurse(&lava, &springs, &mut cache);
            }   
        }
    }

    println!("{}", _sum);
    
}
