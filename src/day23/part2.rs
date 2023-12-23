use std::collections::{HashMap, HashSet};

use crate::utils;


fn dfs(pt: &(usize, usize), end: &(usize, usize), graph: &HashMap<(usize, usize), HashMap<(usize, usize), i32>>, seen: &mut HashSet<(usize, usize)>) -> i32 {
    if pt == end {
        return 0;
    }

    if seen.contains(pt) {
        return -i32::MAX;
    }

    seen.insert(*pt);

    let mut m = -i32::MAX;
    if let Some(neighbors) = graph.get(pt) {
        for (nx, &dist) in neighbors {
            if !seen.contains(nx) {
                m = m.max(dfs(nx, end, graph, seen) + dist);
            }
        }
    }

    seen.remove(pt);
    m
}


pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut forest: Vec<Vec<char>> = Vec::new();
    
    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                let tiles: Vec<char> = _row.chars().collect();
                forest.push(tiles);
            }
        }
    }
    let start: (usize, usize) = (0, forest[0].iter().position(|&p| p == '.').unwrap());
    let end: (usize, usize) = (forest.len()-1, forest[forest.len()-1].iter().position(|&p| p == '.').unwrap());

    let mut points = vec![start, end];

    for (r, row) in forest.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == '#' {
                continue;
            }

            let mut neighbors = 0;
            let neighbor_positions = [(r as isize - 1, c as isize), (r as isize + 1, c as isize), (r as isize, c as isize - 1), (r as isize, c as isize + 1)];

            for &(nr, nc) in &neighbor_positions {
                if nr >= 0 && nr < forest.len() as isize && nc >= 0 && nc < forest[0].len() as isize && forest[nr as usize][nc as usize] != '#' {
                    neighbors += 1;
                }
            }

            if neighbors >= 3 {
                points.push((r, c));
            }
        }
    }

    let mut graph: HashMap<(usize, usize), HashMap<(usize, usize), i32>> = points.iter().map(|&pt| (pt, HashMap::new())).collect();

    for &(sr, sc) in &points {
        let mut stack = vec![(0, sr, sc)];
        let mut seen = HashSet::new();
        seen.insert((sr, sc));

        while let Some((n, r, c)) = stack.pop() {
            if n != 0 && points.contains(&(r, c)) {
                graph.get_mut(&(sr, sc)).unwrap().insert((r, c), n);
                continue;
            }
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (nr, nc) = ((r as isize + dr) as usize, (c as isize + dc) as usize);
                if nr < forest.len() && nc < forest[0].len() && forest[nr][nc] != '#' && !seen.contains(&(nr, nc)) {
                    stack.push((n + 1, nr, nc));
                    seen.insert((nr, nc));
                }
            }
        }
    }

    let mut seen = HashSet::new();
    println!("{}", dfs(&start, &end, &graph, &mut seen));
}
