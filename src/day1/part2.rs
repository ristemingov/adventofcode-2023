use crate::utils;

fn find_numbers(seq: String) -> i32 {
    let mut min: i32 = -1;
    let mut min_num: i32 = -1;
    let mut max: i32 = -1;
    let mut max_num: i32 = -1;

    let collection = [
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ];

    for col in collection {
        let matches: Vec<_> = seq.match_indices(col.0).collect();
        if matches.len() > 0 {
            for mtch in matches {
                let loc = mtch.0 as i32;
                if min == -1 || min > loc {
                    min = loc;
                    min_num = col.1;
                }
                if max == -1 || max < loc {
                    max = loc;
                    max_num = col.1;
                }
            }
        }
    }

    return min_num * 10 + max_num;
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);

    let mut _sum: i32 = 0;
    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                let found = find_numbers(row);
                _sum += found;
            }
        }
    }
    println!("{}", _sum);
}
