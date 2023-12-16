use std::{collections::HashSet, cmp::Ordering};
use std::hash::Hash;
use num::abs;

use crate::utils;

fn unique_pairs<T: Hash + Eq + Ord + Clone>(set: &HashSet<T>) -> HashSet<(T, T)> {
    let mut pairs = HashSet::new();

    for item1 in set {
        for item2 in set {
            match item1.cmp(item2) {
                Ordering::Less => {
                    pairs.insert((item1.clone(), item2.clone()));
                }
                Ordering::Greater => {
                    pairs.insert((item2.clone(), item1.clone()));
                }
                Ordering::Equal => {} // Skip identical pairs
            }
        }
    }

    return pairs;
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut unexpanded_map: Vec<Vec<char>> = Vec::new();
    let mut col_galaxies: HashSet<usize> = HashSet::new();
    let mut row_galaxies: HashSet<usize> = HashSet::new();

    let mut col_count = 0;
    let mut row_count = 0;
    
    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                let mut _row_vec: Vec<char> = _row.chars().collect();

                row_count += 1;
                col_count = _row_vec.len();

                let mut galaxy_found = false;
                for _j in 0.._row_vec.len() {
                    if _row_vec[_j] == '#' {
                        col_galaxies.insert(_j);
                        galaxy_found = true;
                    }
                }
                
                if galaxy_found {
                    row_galaxies.insert(_i);
                }

                unexpanded_map.push(_row_vec);
            }
        }
    }
    
    let mut galaxies: HashSet<(usize, usize)> = HashSet::new();
    let expansion_factor = 1;
    
    let mut vec_expandig_col: Vec<usize> = vec![]; 
    let mut vec_expandig_row: Vec<usize> = vec![]; 

    for _i in 0..row_count {
        if !row_galaxies.contains(&_i) {
            vec_expandig_row.push(_i);
        }
    }

    for _i in 0..col_count {
        if !col_galaxies.contains(&_i) {
            vec_expandig_col.push(_i);
        }
    }
    
    let mut expanding_row: usize = 0;
    for _i in 0..unexpanded_map.len() {
        if vec_expandig_row.contains(&_i) {
            expanding_row += expansion_factor;
        }
        let mut expanding_col: usize = 0;
        for _j in 0..unexpanded_map[_i].len() {
            if vec_expandig_col.contains(&_j) {
                expanding_col += expansion_factor;
            }
            if unexpanded_map[_i][_j] == '#' {
                let col = _j + expanding_col;
                let row = _i + expanding_row;
                galaxies.insert((row, col));
            }
        }
    }


    let pairs = unique_pairs(&galaxies);

    let mut _sum_of_pairs = 0;
    for pair in pairs {
        _sum_of_pairs += abs(pair.1.1 as i32 - pair.0.1 as i32) + abs(pair.1.0 as i32 - pair.0.0 as i32);

    }
    println!("{}", _sum_of_pairs);

}
