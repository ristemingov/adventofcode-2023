use crate::utils;
use std::{collections::HashSet, vec::Vec};

fn rotate_clockwise(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }

    let nrows = matrix.len();
    let ncols = matrix[0].len();
    let mut rotated = vec![Vec::with_capacity(nrows); ncols];

    for row in matrix {
        for (j, &val) in row.iter().enumerate() {
            rotated[j].push(val);
        }
    }

    for row in &mut rotated {
        row.reverse();
    }

    return rotated;
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);

    let mut grid: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                grid.push(_row.chars().collect());
            }
        }
    }
    let mut seen: HashSet<Vec<Vec<char>>> = HashSet::new();
    let mut array: Vec<Vec<Vec<char>>> = Vec::new();
    let mut iteration = 0;
    seen.insert(grid.clone());
    array.push(grid.clone());

    'mainLoop: loop {
        iteration += 1;
        for _k in 0..4 {
            loop {
                let mut x_founded = false;
                for i in 1..grid.len() {
                    for j in 0..grid[i].len() {
                        if (grid[i - 1][j] == '.' && grid[i][j] == 'O')
                            || (grid[i - 1][j] == 'X' && grid[i][j] == 'O')
                        {
                            grid[i - 1][j] = 'O';
                            grid[i][j] = '.';
                            x_founded = true;
                        }
                    }
                }
                if !x_founded {
                    break;
                }
            }
            grid = rotate_clockwise(grid);
        }
        if seen.contains(&grid) {
            break 'mainLoop;
        }
        seen.insert(grid.clone());
        array.push(grid.clone());
    }

    let first = array.iter().position(|x| *x == grid).unwrap();

    grid = array[(1000000000 - first) % (iteration - first) + first].clone();

    let mut _sum = 0;
    for i in 0..grid.len() {
        _sum += grid[i].iter().filter(|&n| *n == 'O').count() * (grid.len() - i);
    }

    println!("{}", _sum);
}
