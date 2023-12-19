use crate::utils;

#[allow(dead_code)]
fn add_points(
    start: (usize, usize),
    dr: i32,
    dc: i32,
    times: i32,
    polygon: &mut Vec<(usize, usize)>,
) -> ((usize, usize), (usize, usize)) {
    let mut r = start.0 as i32;
    let mut c = start.1 as i32;
    let mut max_r = 0;
    let mut max_c = 0;
    for _i in 0..times {
        r += dr;
        c += dc;
        if r < 0 || c < 0 {
            continue;
        }
        if r > max_r {
            max_r = r;
        }
        if c > max_c {
            max_c = c;
        }
        polygon.push((r as usize, c as usize));
    }
    return ((r as usize, c as usize), (max_r as usize, max_c as usize));
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut dig_plan: Vec<Vec<String>> = Vec::new();
    let mut polygon: Vec<(usize, usize)> = Vec::new();

    let mut start: (usize, usize) = (0, 0);
    let mut _current_point = start;
    polygon.push(start);
    let mut dug_cubes = 0;

    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                let dig_instruction: Vec<String> = _row.split(" ").map(|s| s.to_string()).collect();

                let times: i32 = dig_instruction[1].parse().unwrap();
                let mut dr: i32 = 0;
                let mut dc: i32 = 0;
                match dig_instruction[0].as_str() {
                    "U" => {
                        dr = -1;
                        dc = 0;
                    }
                    "D" => {
                        dr = 1;
                        dc = 0;
                    }
                    "L" => {
                        dr = 0;
                        dc = -1;
                    }
                    "R" => {
                        dr = 0;
                        dc = 1;
                    }
                    _ => {
                        println!("Invalid direction: {}", dig_instruction[0]);
                    }
                }

                _current_point = (
                    (start.0 as i32 + (dr * times)) as usize,
                    (start.1 as i32 + (dc * times)) as usize,
                );
                polygon.push(_current_point);
                start = _current_point;
                dug_cubes += times;

                // (start, _) = add_points(start, dr, dc, times, &mut polygon);
                dig_plan.push(dig_instruction);
            }
        }
    }

    // remove the last point as its the same as the first
    // polygon.pop();

    let mut area_sum: i32 = 0;
    area_sum += (polygon[0].0 * (polygon[polygon.len() - 1].1 - polygon[0].1)) as i32;

    for i in 1..polygon.len() {
        area_sum += (polygon[i].0 as i32
            * (polygon[i - 1].1 as i32 - polygon[(i + 1) % polygon.len()].1 as i32))
            as i32;
    }
    area_sum = area_sum.abs();
    area_sum /= 2;
    println!("Area: {}", area_sum);
    println!("Boundary points: {:?}", dug_cubes);
    println!("Sum: {}", (area_sum - (dug_cubes / 2) + 1) + dug_cubes);

    // A try with the ray casting algorithm

    // println!("Max row: {}", max_r+1);
    // println!("Max col: {}", max_c+1);
    // let mut diff = 0;

    // for i in 0..max_r+1 {
    //     let mut count_entries = 0;
    //     let mut state_changed = false;
    //     for j in 0..max_c+1 {
    //         let mut char_to_print = ".";

    //         if polygon.contains(&(i, j)) {
    //             char_to_print = "#";
    //             state_changed = true;
    //         } else {
    //             char_to_print = ".";
    //             if state_changed {
    //                 count_entries += 1;
    //                 state_changed = false;
    //             }
    //         }

    //         if count_entries % 2 == 1 {
    //             char_to_print = "#";
    //         }
    //         if char_to_print == "." {
    //             diff += 1;
    //         }
    //         print!("{}", char_to_print);
    //     }
    //     println!("");
    // }

    // println!("Sum: {}", ((max_r+1)*(max_c+1))-diff);
}
