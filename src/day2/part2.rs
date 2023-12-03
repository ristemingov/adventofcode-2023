use std::collections::HashMap;
use crate::utils;

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);

    let mut _sum: i32 = 0;
    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for line in lines {
            if let Ok(row) = line {
                let v: Vec<&str> = row.split(':').collect();
                let sets: Vec<&str> = v[1].split(';').collect();

                let mut _mins: HashMap<&str, i32> = HashMap::from([
                    ("red", 0),
                    ("green", 0),
                    ("blue", 0),
                ]);

                for set in sets {
                    let cubes: Vec<&str> = set.split(',').collect();

                    for color in cubes {
                        let tup: Vec<&str> = color.trim().split(' ').collect();
                        let num_cubes: i32 = tup[0].parse().unwrap();
                        if _mins.get(&tup[1]).unwrap() <= &num_cubes {
                            *_mins.get_mut(&tup[1]).unwrap() = num_cubes;
                        }
                    }
                }

                _sum +=
                    _mins.get(&"red").unwrap() *
                    _mins.get(&"green").unwrap() *
                    _mins.get(&"blue").unwrap();
            }
        }
    }
    println!("{}", _sum);
}
