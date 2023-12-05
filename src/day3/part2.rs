use crate::utils;
use std::collections::HashMap;

fn is_star_symbol(ch: char) -> bool {
    return ch == '*';
}

fn log_part(gears: &mut HashMap<String, Vec<i32>>, key: String, part_num: i32) {
    if gears.contains_key(&key) {
        let parts = gears.get_mut(&key);
        if let Some(p) = parts {
            p.push(part_num);
        }
    } else {
        gears.insert(key, vec![part_num]);
    }
}

fn has_proximity(
    matrix: &Vec<Vec<char>>,
    gears: &mut HashMap<String, Vec<i32>>,
    start_idx: usize,
    end_idx: usize,
    row_num: usize,
    part_num: i32
) -> bool {
    for (i, mat) in matrix.iter().enumerate() {
        if start_idx > 0 {
            if is_star_symbol(mat[start_idx - 1]) {
                log_part(
                    gears,
                    format!("{}-{}", (row_num - 1 + i).to_string(), (start_idx - 1).to_string()),
                    part_num
                );
            }
        }
        for n in start_idx..=end_idx {
            if is_star_symbol(mat[n]) {
                log_part(
                    gears,
                    format!("{}-{}", (row_num - 1 + i).to_string(), n.to_string()),
                    part_num
                );
            }
        }
        if end_idx < mat.len() - 1 {
            if is_star_symbol(mat[end_idx + 1]) {
                log_part(
                    gears,
                    format!("{}-{}", (row_num - 1 + i).to_string(), (end_idx + 1).to_string()),
                    part_num
                );
            }
        }
    }
    return false;
}

fn find_parts(
    matrix: &Vec<Vec<char>>,
    gears: &mut HashMap<String, Vec<i32>>,
    row_num: usize
) -> Vec<i32> {
    let mut temp: String = "".to_string();
    let mut idcs: Vec<usize> = vec![];
    let mut parts: Vec<i32> = vec![];
    for (i, ch) in matrix[1].iter().enumerate() {
        if !ch.is_digit(10) || i == matrix[1].len() - 1 {
            if ch.is_digit(10) {
                temp.push(*ch);
                idcs.push(i);
            }
            if temp != "" {
                let part_num: i32 = temp.parse().unwrap();
                if has_proximity(&matrix, gears, idcs[0], idcs[idcs.len() - 1], row_num, part_num) {
                    parts.push(part_num);
                }
            }
            temp = "".to_string();
            idcs = vec![];
        } else {
            temp.push(*ch);
            idcs.push(i);
        }
    }
    return parts;
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let row_size: usize = 140;
    let mut gears: HashMap<String, Vec<i32>> = HashMap::new();

    let mut _sum: i32 = 0;
    let mut row_num: usize = 0;
    if let Ok(lines) = utils::misc::read_lines(file_path) {
        let mut matrix: Vec<Vec<char>> = Vec::new();

        matrix.push(vec!['.'; row_size]);

        for (i, line) in lines.enumerate() {
            if let Ok(row) = line {
                matrix.push(row.chars().collect());
                if i == 0 {
                    continue;
                }
                let _parts = find_parts(&matrix, &mut gears, i);

                if matrix.len() == 3 {
                    matrix.remove(0);
                }
                row_num += 1;
            }
        }
        matrix.push(vec!['.'; row_size]);
        let _parts = find_parts(&matrix, &mut gears, row_num + 1);

        for (_key, value) in &gears {
            if value.len() == 2 {
                _sum += value[0] * value[1];
            }
        }

        println!("{}", _sum);
    }
}
