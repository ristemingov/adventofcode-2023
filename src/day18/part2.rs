use std::i64;

use crate::utils;

fn convert_hex_to_instruction(hex_string: &str) -> (String, i64) {
    let hex_chars: Vec<char> = hex_string.chars().collect();
    let mut direction = String::new();
    match hex_chars[hex_chars.len() - 1] {
        '0' => direction = "R".to_string(),
        '1' => direction = "D".to_string(),
        '2' => direction = "L".to_string(),
        '3' => direction = "U".to_string(),
        _ => println!("Invalid hex char: {}", hex_chars[hex_chars.len() - 1]),
        
    }
    let real_hex = hex_chars[1..hex_chars.len() - 1].iter().collect::<String>();
    if let Ok(times) = i64::from_str_radix(&real_hex , 16) {
        return (direction, times);
    }
    return (direction, 0);
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut polygon: Vec<(i64, i64)> = Vec::new();

    let mut start: (i64, i64) = (0, 0);
    let mut _current_point = start;
    polygon.push(start);
    let mut dug_cubes = 0;

    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                let dig_instruction: Vec<String> = _row.split(" ").map(|s| s.to_string()).collect();
                let hex_instruction = dig_instruction[2].replace("(", "").replace(")", "");
                let (direction, times) = convert_hex_to_instruction(&hex_instruction);
            
                let mut dr: i64 = 0;
                let mut dc: i64 = 0;

                match direction.as_str() {
                    "U" => {
                        dr = -1;
                        dc = 0;
                    }
                    "D" => {
                        dr = 1;
                        dc = 0;
                    }
                    "L" => {
                        dr = 0;
                        dc = -1;
                    }
                    "R" => {
                        dr = 0;
                        dc = 1;
                    }
                    _ => {
                        println!("Invalid direction: {}", dig_instruction[0]);
                    }
                }

                _current_point = (
                    (start.0 + (dr * times)),
                    (start.1 + (dc * times)),
                );
                polygon.push(_current_point);
                start = _current_point;
                dug_cubes += times;

            }
        }
    }

    let mut area_sum = 0;
    area_sum += polygon[0].0 * (polygon[polygon.len() - 1].1 - polygon[0].1);

    for i in 1..polygon.len() {

        area_sum += polygon[i].0
            * (polygon[i - 1].1 - polygon[(i + 1) % polygon.len()].1);
    }
    area_sum = area_sum.abs();
    area_sum /= 2;

    println!("Area: {}", area_sum);
    println!("Boundary points: {:?}", dug_cubes);
    println!("Sum: {}", (area_sum - (dug_cubes / 2) + 1) + dug_cubes);


}
