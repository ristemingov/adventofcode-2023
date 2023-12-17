use crate::utils;
use std::cmp;
use itertools::izip;


fn transpose_strings(vec_of_strings: Vec<String>) -> Vec<String> {
    let char_vecs: Vec<Vec<char>> = vec_of_strings
        .into_iter()
        .map(|s| s.chars().collect())
        .collect();
    let max_len = char_vecs.iter().map(|v| v.len()).max().unwrap_or(0);
    return (0..max_len)
        .map(|i| {
            char_vecs
                .iter()
                .map(|v| *v.get(i).unwrap_or(&' '))
                .collect::<String>()
        })
        .collect();
}

fn get_reflection(matrix: &Vec<String>) -> i32 {
    for i in 1..matrix.len() {
        let mut top: Vec<String> = matrix[i..].to_vec();
        let mut bottom: Vec<String> = matrix[..i].to_vec();
        bottom.reverse();
        let min = cmp::min(top.len(), bottom.len());
        top = top[..min].to_vec();
        bottom = bottom[..min].to_vec();

        // println!("{:?}", izip![top, bottom].collect::<Vec<(String, String)>>());
        let mut _sum: i32 = 0;
        for (t, b) in izip![top, bottom] {
            let mut _count: i32 = 0;
            for (t_c, b_c) in izip![t.chars(), b.chars()] {
                if t_c != b_c {
                    _count += 1;
                }
            }
            _sum += _count;
        }
        if _sum == 1 {
            return i as i32;
        }
        
    }
    return 0;
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut grid: Vec<Vec<String>> = Vec::new();
    let mut matrix: Vec<String> = Vec::new();

    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                let mat_row: String = _row;
                if mat_row.len() != 0 {
                    matrix.push(mat_row);
                    continue;
                }
                grid.push(matrix);
                matrix = Vec::new();
            }
        }
        grid.push(matrix);
    }

    let mut _sum: i32 = 0;

    for g in grid.iter() {
        _sum += get_reflection(g) * 100;
        _sum += get_reflection(&transpose_strings(g.to_vec()));
    }

    println!("{}", _sum);
}
