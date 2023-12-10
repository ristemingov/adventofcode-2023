use std::env;

mod utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let part = &args[2];

    let file_path: String = format!("data/day{}/input.txt", day);

    let day_num: i32 = day.parse().unwrap();
    let part_num: i32 = part.parse().unwrap();

    match day_num {
        1 =>
            match part_num {
                1 => day1::part1::solve(&file_path),
                2 => day1::part2::solve(&file_path),
                _ => panic!("Invalid part for day: {}. Selected part {}", day, part),
            }
        2 =>
            match part_num {
                1 => day2::part1::solve(&file_path),
                2 => day2::part2::solve(&file_path),
                _ => panic!("Invalid part for day: {}. Selected part {}", day, part),
            }

        3 =>
            match part_num {
                1 => day3::part1::solve(&file_path),
                2 => day3::part2::solve(&file_path),
                _ => panic!("Invalid part for day: {}. Selected part {}", day, part),
            }
        
        4 =>
            match part_num {
                1 => day4::part1::solve(&file_path),
                2 => day4::part2::solve(&file_path),
                _ => panic!("Invalid part for day: {}. Selected part {}", day, part),
            }
        
        5 =>
            match part_num {
                1 => day5::part1::solve(&file_path),
                2 => day5::part2::solve(&file_path),
                _ => panic!("Invalid part for day: {}. Selected part {}", day, part),
            }

        _ => panic!("Invalid day selected: {}.", day),
    }
}
