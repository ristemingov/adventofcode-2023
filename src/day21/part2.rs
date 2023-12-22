use std::collections::HashSet;

use crate::utils;

fn is_position_possible(
    garden: &Vec<Vec<char>>,
    position: (usize, usize),
    visited: &HashSet<(usize, usize)>,
) -> bool {
    return position.0 < garden.len()
        && position.1 < garden[position.0].len()
        && !visited.contains(&position)
        && garden[position.0][position.1] != '#';
}

fn find_tiles(garden: &Vec<Vec<char>>, start: (usize, usize), step: i64) -> i64 {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut answers: HashSet<(usize, usize)> = HashSet::new();

    let mut queue: Vec<(usize, usize, i64)> = Vec::new();
    queue.push((start.0, start.1, step));
    visited.insert(start);

    while !queue.is_empty() {
        let position = queue.remove(0);
        if position.2 % 2 == 0 {
            answers.insert((position.0, position.1));
        }
        if position.2 == 0 {
            continue;
        }

        let next_positions: Vec<(usize, usize)> = [(0, -1), (0, 1), (-1, 0), (1, 0)]
            .iter()
            .map(|f: &(i32, i32)| {
                (
                    (position.0 as i32 + f.0) as usize,
                    (position.1 as i32 + f.1) as usize,
                )
            })
            .collect();

        for next_position in next_positions {
            if is_position_possible(garden, next_position, &visited) {
                visited.insert((next_position.0, next_position.1));
                queue.push((next_position.0, next_position.1, position.2 - 1));
            }
        }
    }

    return answers.len().try_into().unwrap();
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut garden: Vec<Vec<char>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                let tiles: Vec<char> = _row.chars().collect();
                garden.push(tiles.clone());
                let position = match tiles.iter().position(|&r| r == 'S') {
                    Some(x) => x,
                    None => continue,
                };

                start = (_i, position);
            }
        }
    }

    let step: i64 = 26501365;
    let size: i64 = garden.len() as i64;
    let _sum = find_tiles(&garden, start, step);

    let grid_width = step / size - 1;

    let odd = (grid_width / 2 * 2 + 1).pow(2);
    let even = ((grid_width + 1) / 2 * 2).pow(2);

    let odd_points = find_tiles(&garden, (start.0, start.1), size * 2 + 1);
    let even_points = find_tiles(&garden, (start.0, start.1), size * 2);

    let corner_t = find_tiles(&garden, ((size - 1) as usize, start.1), size - 1);
    let corner_r = find_tiles(&garden, (start.0, 0), size - 1);
    let corner_b = find_tiles(&garden, (0, start.1), size - 1);
    let corner_l = find_tiles(&garden, (start.0, (size - 1) as usize), size - 1);

    let small_tr = find_tiles(&garden, ((size - 1) as usize, 0), size / 2 - 1);
    let small_tl = find_tiles(
        &garden,
        ((size - 1) as usize, (size - 1) as usize),
        size / 2 - 1,
    );
    let small_br = find_tiles(&garden, (0, 0), size / 2 - 1);
    let small_bl = find_tiles(&garden, (0, (size - 1) as usize), size / 2 - 1);

    let large_tr = find_tiles(&garden, ((size - 1) as usize, 0), size * 3 / 2 - 1);
    let large_tl = find_tiles(
        &garden,
        ((size - 1) as usize, (size - 1) as usize),
        size * 3 / 2 - 1,
    );
    let large_br = find_tiles(&garden, (0, 0), size * 3 / 2 - 1);
    let large_bl = find_tiles(&garden, (0, (size - 1) as usize), size * 3 / 2 - 1);

    println!(
        "{}",
        (odd * odd_points
            + even * even_points
            + corner_t
            + corner_r
            + corner_b
            + corner_l
            + (grid_width + 1) * (small_tr + small_tl + small_br + small_bl)
            + grid_width * (large_tr + large_tl + large_br + large_bl))
    );
}
