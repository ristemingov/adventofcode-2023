use std::env;

mod utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

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
        
        6 =>
            match part_num {
                1 => day6::part1::solve(&file_path),
                2 => day6::part2::solve(&file_path),
                _ => panic!("Invalid part for day: {}. Selected part {}", day, part),
            }
        
        7 => 
            match part_num {
                1 => day7::part1::solve(&file_path),
                2 => day7::part2::solve(&file_path),
                _ => panic!("Invalid part for day: {}. Selected part {}", day, part),
            }
        
        8 => 
            match part_num {
                1 => day8::part1::solve(&file_path),
                2 => day8::part2::solve(&file_path),
                _ => panic!("Invalid part for day: {}. Selected part {}", day, part),
            }
        
        9 => 
            match part_num {
                1 => day9::part1::solve(&file_path),
                2 => day9::part2::solve(&file_path),
                _ => panic!("Invalid part for day: {}. Selected part {}", day, part),
            }
        
        10 =>
            match part_num {
                1 => day10::part1::solve(&file_path),
                2 => day10::part2::solve(&file_path),
                _ => panic!("Invalid part for day: {}. Selected part {}", day, part),
            }
        
        11 => 
            match part_num {
                1 => day11::part1::solve(&file_path),
                2 => day11::part2::solve(&file_path),
                _ => panic!("Invalid part for day: {}. Selected part {}", day, part),
            }
        
        12 =>
            match part_num {
                1 => day12::part1::solve(&file_path),
                2 => day12::part2::solve(&file_path),
                _ => panic!("Invalid part for day: {}. Selected part {}", day, part),
            }
        
        13 =>
            match part_num {
                1 => day13::part1::solve(&file_path),
                2 => day13::part2::solve(&file_path),
                _ => panic!("Invalid part for day: {}. Selected part {}", day, part),
            }
        
        14 =>
            match part_num {
                1 => day14::part1::solve(&file_path),
                2 => day14::part2::solve(&file_path),
                _ => panic!("Invalid part for day: {}. Selected part {}", day, part),
            }
        
        15 =>
            match part_num {
                1 => day15::part1::solve(&file_path),
                2 => day15::part2::solve(&file_path),
                _ => panic!("Invalid part for day: {}. Selected part {}", day, part),
            }
        
        16 =>
            match part_num {
                1 => day16::part1::solve(&file_path),
                2 => day16::part2::solve(&file_path),
                _ => panic!("Invalid part for day: {}. Selected part {}", day, part),
            }
        
        17 =>
            match part_num {
                1 => day17::part1::solve(&file_path),
                2 => day17::part2::solve(&file_path),
                _ => panic!("Invalid part for day: {}. Selected part {}", day, part),
            }


        _ => panic!("Invalid day selected: {}.", day),
    }
}
