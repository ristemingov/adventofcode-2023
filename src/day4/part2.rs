use std::collections::HashSet;
use std::iter::FromIterator;

use crate::utils;

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut _sum: i32 = 0;
    let mut count_vec: Vec<i32> = Vec::new();
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
                let intersection: HashSet<_> = winning_set
                    .intersection(&my_selection_set)
                    .cloned()
                    .collect();
                let mut _get: i32 = 0;

                match count_vec.get(_i) {
                    Some(x) => {
                        _get = *x;
                    }
                    None => {
                        _get = 1;
                        count_vec.push(1);
                    }
                }

                for _m in 0.._get {
                    for k in 1..=intersection.len() {
                        match count_vec.get(_i + k) {
                            Some(_x) => {
                                count_vec[_i + k] += 1;
                            }
                            None => {
                                count_vec.push(2);
                            }
                        }
                    }
                }
            }
        }
    }
    for i in count_vec {
        _sum += i;
    }
    println!("{}", _sum);
}
