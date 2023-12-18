use std::collections::HashSet;

use crate::utils;


fn add_to_queue(queue: &mut Vec<(i32, i32, i32, i32)>, seen: &mut HashSet<(i32, i32, i32, i32)>, x: i32, y: i32, z: i32, w: i32) {
    if seen.contains(&(x, y, z, w)) {
        return;
    }
    seen.insert((x, y, z, w));
    queue.push((x, y, z, w));
}

fn energize(tiles: &Vec<Vec<char>>, fr: i32, fc: i32, fdr: i32, fdc: i32) -> i32 {
    let mut queue: Vec<(i32, i32, i32, i32)> = Vec::new();
    let mut seen: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    queue.push((fr, fc, fdr, fdc));
    while !queue.is_empty() {
      let (mut r, mut c, mut dr, mut dc) = queue.remove(0);

      r += dr;
      c += dc;

      if r < 0 || r >= tiles.len() as i32 || c < 0 || c >= tiles[0].len() as i32 {
        continue;
      }

      let ch = tiles[r as usize][c as usize];

      if ch == '.' || (ch == '-' && dc!=0) || ( ch == '|' && dr!=0) {
        add_to_queue(&mut queue, &mut seen, r, c, dr, dc);
      } else if ch == '/' {
        let temp = (-dc, -dr);
        dr = temp.0;
        dc = temp.1;
        add_to_queue(&mut queue, &mut seen, r, c, dr, dc);
      } else if ch == '\\' {
        let temp = (dc, dr);
        dr = temp.0;
        dc = temp.1;
        add_to_queue(&mut queue, &mut seen, r, c, dr, dc);
      } else {
        if ch == '|' {
          for (dr, dc) in vec![(1, 0), (-1, 0)] {
            add_to_queue(&mut queue, &mut seen, r, c, dr, dc);
          }
        } else {
            for (dr, dc) in vec![(0, 1), (0, -1)] {
                add_to_queue(&mut queue, &mut seen, r, c, dr, dc);
            }
        }
      }
    
    }
    let mut coords: HashSet<(i32, i32)> = HashSet::new();
    for (r, c, _, _) in seen {
        coords.insert((r, c));
    }

    return coords.len() as i32;
}


pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut tiles: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                tiles.push(_row.chars().collect());
            }
        }
    }

    let mut max = 0;
    for r in 0..tiles.len() {
        max = std::cmp::max(max, energize(&tiles, r as i32, -1, 0, 1));
        max = std::cmp::max(max, energize(&tiles, r as i32, tiles[0].len() as i32, 0, -1));
    }

    for c in 0..tiles[0].len() {
        max = std::cmp::max(max, energize(&tiles, -1, c as i32, 1, 0));
        max = std::cmp::max(max, energize(&tiles, tiles.len() as i32, c as i32, -1, 0));
    }
    println!("{}", max);
}
