use crate::utils;
use std::collections::{HashSet, HashMap};


fn is_connection_pipe(current_pipe: char, next_pipe: char, relative_row: i32, relative_col: i32) -> bool {
    
    let piping_map: HashMap<&str, Vec<char>> = HashMap::from([
        ("S-10", vec!['|', '7', 'F']),
        ("S0-1", vec!['-', 'F', 'L']),
        ("S01", vec!['-', '7', 'J']),
        ("S10", vec!['|', 'J', 'L']),

        ("|-10", vec!['|', 'F', '7']),
        ("|0-1", vec![]),
        ("|01", vec![]),
        ("|10", vec!['|', 'J', 'L']),

        ("--10", vec![]),
        ("-0-1", vec!['-', 'F', 'L']),
        ("-01", vec!['-', '7', 'J']),
        ("-10", vec![]),

        ("L-10", vec!['|', '7', 'F']),
        ("L0-1", vec![]),
        ("L01", vec!['-', '7', 'J']),
        ("L10", vec![]),

        ("J-10", vec!['|', '7', 'F']),
        ("J0-1", vec!['-', 'F', 'L']),
        ("J01", vec![]),
        ("J10", vec![]),

        ("7-10", vec![]),
        ("70-1", vec!['-', 'F', 'L']),
        ("701", vec![]),
        ("710", vec!['|', 'J', 'L']),

        ("F-10", vec![]),
        ("F0-1", vec![]),
        ("F01", vec!['-', '7', 'J']),
        ("F10", vec!['|', 'J', 'L']),

    ]);
    let key: &str = &format!("{}{}{}", current_pipe, relative_row, relative_col);
    return piping_map.get(key).unwrap().contains(&next_pipe);
}


fn get_next_directions(pipe_map: &Vec<Vec<char>>, current_row: usize, current_col: usize, step: usize, visited: &HashSet<(usize, usize)>) -> Vec<(usize, usize, usize)> {
    let mut directions: Vec<(usize, usize, usize)> = Vec::new();
    let current_pipe: char = pipe_map[current_row][current_col];
    
    let possible_directions: Vec<(i32, i32)> = vec![
        (-1, 0), 
        (0,-1), 
        (0, 1), 
        (1, 0)];

    for direction in possible_directions {
        let next_row: i32 = current_row as i32 + direction.0;
        let next_col: i32 = current_col as i32 + direction.1;
        let next_direction: (usize, usize) = (next_row as usize, next_col as usize);
        if visited.contains(&next_direction) {
            continue;
        }
        
        if next_row >= pipe_map.len() as i32 || next_col >= pipe_map[0].len() as i32 {
            continue;
        }

        if next_row < 0 || next_col < 0 {
            continue;
        }

        let next_pipe: char = pipe_map[(current_row as i32 + direction.0) as usize][(current_col as i32 + direction.1) as usize];
        if is_connection_pipe(current_pipe, next_pipe, direction.0, direction.1) {
            directions.push((next_direction.0, next_direction.1, step + 1));
        }

    }
    return directions;
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);

    let mut pipe_map: Vec<Vec<char>> = Vec::new();
    let mut start_row_index: usize = 0;
    let mut start_col_index: usize = 0;

    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                let pipe_line: Vec<char> = _row.chars().collect();
                let start_index = &pipe_line.iter().position(|&r| r == 'S');
                pipe_map.push(pipe_line);
                if start_index.is_none() {
                    continue;
                }
                start_row_index = _i;
                start_col_index = start_index.unwrap();
            }
        }
    }
    let mut dirrection_queue: Vec<(usize, usize, usize)> = vec![(start_row_index, start_col_index, 0)];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut max_step_count: usize = 0;

    while dirrection_queue.len() > 0 {
        let dir = dirrection_queue.remove(0);
        visited.insert((dir.0, dir.1));
        let found_directions = get_next_directions(&pipe_map, dir.0, dir.1, dir.2, &visited);
        for found_dir in found_directions {
            if found_dir.0 == start_row_index && found_dir.1 == start_col_index {
                continue;
            }
            if found_dir.2 > max_step_count {
                max_step_count = found_dir.2;
            }
            dirrection_queue.push(found_dir);
        }
    }
    println!("{} ", max_step_count);
    
}
