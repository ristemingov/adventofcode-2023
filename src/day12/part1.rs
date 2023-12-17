use crate::utils;

fn generate_combinations(s: &str) -> Vec<String> {
    let mut results = Vec::new();
    generate_combinations_helper(s, 0, &mut String::new(), &mut results);
    results
}

fn generate_combinations_helper(s: &str, index: usize, current: &mut String, results: &mut Vec<String>) {
    if index == s.len() {
        results.push(current.clone());
        return;
    }

    match s.chars().nth(index).unwrap() {
        '?' => {
            for &c in ['.', '#'].iter() {
                current.push(c);
                generate_combinations_helper(s, index + 1, current, results);
                current.pop();
            }
        }
        other => {
            current.push(other);
            generate_combinations_helper(s, index + 1, current, results);
            current.pop();
        }
    }
}

fn is_possible(map: Vec<char>, calculations: Vec<i32>) -> bool {
    let s: String = map.iter().collect();
    let calc:Vec<usize> = s.split(".").filter(|x| x.len() > 0).map(|f| f.len()).collect::<Vec<usize>>();
    if calc.len() != calculations.len() {
        return false;
    }
    for i in 0..calc.len() {
        if calc[i] != calculations[i] as usize {
            return false;
        }
    }
    return true;
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut hot_springs_map: Vec<Vec<char>> = Vec::new();
    let mut hot_springs_calculations: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                let inputs: Vec<&str> = _row.split(" ").collect();
                hot_springs_map.push(inputs[0].chars().collect());
                hot_springs_calculations.push(inputs[1].split(",").filter_map(|s| s.parse().ok()).collect());
            }
        }
    }

    let mut _sum: i32 = 0;
    for i in 0..hot_springs_map.len() {
        let combinations = generate_combinations(hot_springs_map[i].iter().collect::<String>().as_str());
        for combo in combinations {
            if is_possible(combo.chars().collect(), hot_springs_calculations[i].clone()) {
                _sum += 1;
            }
        }
    }
    println!("{}", _sum);
    
}
