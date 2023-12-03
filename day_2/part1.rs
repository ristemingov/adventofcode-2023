use std::env;
use std::collections::HashMap;
mod utils;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);
    let _possible: HashMap<&str, i32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let mut _sum: i32 = 0;
    if let Ok(lines) = utils::read_lines(file_path) {
        'lineLoop: for line in lines {
            if let Ok(row) = line {
                let v: Vec<&str> = row.split(':').collect();
                let g_s: Vec<&str> = v[0].split(' ').collect();
                let game: i32 = g_s[1].parse().unwrap();
                let sets: Vec<&str> = v[1].split(';').collect();

                for set in sets {
                    let cubes: Vec<&str> = set.split(',').collect();

                    for color in cubes {
                        let tup: Vec<&str> = color.trim().split(' ').collect();
                        let num_cubes: i32 = tup[0].parse().unwrap();
                        if _possible.get(&tup[1]).unwrap() < &num_cubes {
                            continue 'lineLoop
                        }

                    }
                   
                }
                
                _sum += game;
                
            }
        }
    }
    println!("{}", _sum);
}
