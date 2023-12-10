use std::collections::HashSet;

use crate::utils;

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut _sum: i32 = 0;
    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(row) = _line {
                let v: Vec<&str> = row.trim().split(':').collect();
                let sets: Vec<&str> = v[1].trim().split('|').collect();
                let winning_vec: Vec<&str> = sets[0].trim().split(' ').collect();
                let my_selection_vec: Vec<&str> = sets[1].trim().split(' ').collect();
                let winning_set: HashSet<&str> = HashSet::from_iter(
                    winning_vec
                        .iter()
                        .filter(|num| str::parse::<i32>(num).is_ok())
                        .cloned(),
                );
                let my_selection_set: HashSet<&str> = HashSet::from_iter(
                    my_selection_vec
                        .iter()
                        .filter(|num| str::parse::<i32>(num).is_ok())
                        .cloned(),
                );
                let intersection = winning_set.intersection(&my_selection_set);
                let mut _exp: u32 = 0;
                let intersection_size: usize = intersection.count();
                if intersection_size > 0 {
                    _exp = intersection_size as u32 - 1;
                    _sum += 2_i32.pow(_exp);
                }
            }
        }
    }
    println!("{}", _sum);
}
