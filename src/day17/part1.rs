use crate::utils;
use priority_queue::PriorityQueue;
use std::{collections::HashSet, cmp::Reverse};


fn dijkstra(graph: &Vec<Vec<i32>>, start: (usize, usize), goal: (usize, usize)) -> i32 {
    let mut heap: PriorityQueue<(i32, usize, usize, i32, i32, i32), (Reverse<i32>, usize, usize, i32, i32, i32) > = PriorityQueue::new();
    let mut seen: HashSet<(usize, usize, i32, i32, i32)> = HashSet::new();
    let mut found_loss = 0;

    heap.push(( 0, start.0, start.1, 0, 0, 0), (Reverse(0), start.0, start.1, 0, 0, 0));

    while !heap.is_empty()
    {   
        let node = match heap.pop() {
            Some(n) => n,
            None => break,
        };
        let Reverse(heat_loss) = node.1.0;
        let r = node.0.1;
        let c = node.0.2;
        let dr = node.0.3;
        let dc = node.0.4;
        let direction_count = node.0.5;

        if (r,c) == goal {
            found_loss = heat_loss;
            break;
        }

        if r >= graph.len() || c >= graph[0].len() {
            continue;
        }

        if seen.contains(&(r, c, dr, dc, direction_count)) {
            continue;
        }

        seen.insert((r, c, dr, dc, direction_count));

        if direction_count < 3 && (dr, dc) != (0, 0) {
            let nr = (r as i32 + dr) as usize;
            let nc = (c as i32 + dc) as usize;
            if nr < graph.len() && nc < graph[0].len() {
                //println!("Current {}, Next {}", heat_loss, graph[nr][nc]);
                heap.push((
                    heat_loss + graph[nr][nc],
                    nr,
                    nc,
                    dr,
                    dc,
                    direction_count + 1),
                    (Reverse(heat_loss + graph[nr][nc]), nr, nc, dr, dc, direction_count + 1));
            }
        }

        for (ndr, ndc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
          if (ndr, ndc) != (dr, dc) && (ndr, ndc) != (-dr, -dc) {
            let nr = (r as i32 + ndr) as usize;
            let nc = (c as i32 + ndc) as usize;
            if nr < graph.len() && nc < graph[0].len() {
                // println!("Current {}, Next {}", heat_loss, graph[nr][nc]);
                  heap.push((
                    heat_loss + graph[nr][nc],
                    nr,
                    nc,
                    ndr,
                    ndc,
                    1,
                  ), (Reverse(heat_loss + graph[nr][nc]), nr, nc, ndr, ndc, 1));
            }
          }
        }
    }
    return found_loss;
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut grid: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                let mut digits = vec![];
                for r in _row.chars() {
                    if r.is_digit(10) {
                        digits.push((r as i32) - 0x30);
                    }
                }
                grid.push(digits);
            }
        }
    }
    
    let sum = dijkstra(
        &grid,
        (0, 0),
        (grid.len() - 1, grid[grid.len() - 1].len() - 1),
    );
    println!("{}", sum);
}
