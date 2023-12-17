use crate::utils;

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
 
    loop {
        let mut x_founded = false;
        for i in 1..grid.len() {
            for j in 0..grid[i].len() {
                if (grid[i-1][j] == '.' && grid[i][j] == 'O') || (grid[i-1][j] == 'X' && grid[i][j] == 'O') {
                    grid[i-1][j] = 'O';
                    grid[i][j] = '.';
                    x_founded = true;
                }
            }
        }
        if !x_founded{
            break;
        }
    }
    let mut _sum = 0;
    for i in 0..grid.len() {
        _sum += grid[i].iter().filter(|&n| *n == 'O').count() * (grid.len() - i);
    }

    println!("{}", _sum);
}
