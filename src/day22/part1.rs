use std::collections::HashSet;

use crate::utils;

fn overlaps(brick: &Vec<usize>, check: &Vec<usize>) -> bool {
    return std::cmp::max(brick[0], check[0]) <= std::cmp::min(brick[3], check[3])
        && std::cmp::max(brick[1], check[1]) <= std::cmp::min(brick[4], check[4]);
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut bricks: Vec<Vec<usize>> = Vec::new();
    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                let brick: Vec<usize> = _row
                    .replace("~", ",")
                    .split(",")
                    .filter_map(|s| s.parse().ok())
                    .collect();
                bricks.push(brick);
            }
        }
    }
    bricks.sort_by(|a, b| (a[2]).partial_cmp(&b[2]).unwrap());
    let mut _sum = 0;

    for index in 0..bricks.len() {
        let mut max_z = 1;
        for check_index in 0..index {
            if overlaps(&bricks[index], &bricks[check_index]) {
                max_z = std::cmp::max(max_z, bricks[check_index][5] + 1);
            }
        }
        bricks[index][5] -= bricks[index][2] - max_z;
        bricks[index][2] = max_z;
    }
    bricks.sort_by(|a, b| (a[2]).partial_cmp(&b[2]).unwrap());

    let mut k_supports_v = vec![HashSet::new(); bricks.len()];
    let mut v_supports_k = vec![HashSet::new(); bricks.len()];

    for (j, upper) in bricks.iter().enumerate() {
        for (i, lower) in bricks.iter().enumerate().take(j) {
            if overlaps(&lower, &upper) && upper[2] == lower[5] + 1 {
                k_supports_v[i].insert(j);
                v_supports_k[j].insert(i);
            }
        }
    }

    let mut total = 0;
    for i in 0..bricks.len() {
        if k_supports_v[i].iter().all(|&j| v_supports_k[j].len() >= 2) {
            total += 1;
        }
    }

    println!("{}", total);
}
