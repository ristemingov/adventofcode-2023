use crate::utils;

fn construct_map(lines: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut constucted_map: Vec<Vec<u64>> = Vec::new();

    for row in lines {
        if row.len() < 3 {
            continue;
        }
        let source_start = row[1];
        let destination_stat = row[0];
        constucted_map.push(vec![
            source_start,
            source_start + row[2],
            destination_stat,
            destination_stat + row[2],
        ]);
    }
    return constucted_map;
}

fn lookup_map(map: &Vec<Vec<u64>>, value: u64) -> u64 {
    for row in map {
        if row[0] <= value && row[1] >= value {
            return row[2] + (value - row[0]);
        }
    }
    return value;
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut _sum: i32 = 0;

    let mut seeds = Vec::new();
    let mut seed_to_soil_map = Vec::new();
    let mut soil_to_fertilizer_map = Vec::new();
    let mut fertilizer_to_water_map = Vec::new();
    let mut water_to_light_map = Vec::new();
    let mut light_to_temperature_map = Vec::new();
    let mut temperature_to_humidity_map = Vec::new();
    let mut humidity_to_location_map = Vec::new();

    let mut current_map = "seeds";

    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(row) = _line {
                if row.starts_with("seeds:") {
                    current_map = "seeds";
                    let numbers: Vec<u64> = row
                        .split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    seeds.extend(numbers);
                } else if row.starts_with("seed-to-soil map:") {
                    current_map = "seed_to_soil";
                } else if row.starts_with("soil-to-fertilizer map:") {
                    current_map = "soil_to_fertilizer";
                } else if row.starts_with("fertilizer-to-water map:") {
                    current_map = "fertilizer_to_water";
                } else if row.starts_with("water-to-light map:") {
                    current_map = "water_to_light";
                } else if row.starts_with("light-to-temperature map:") {
                    current_map = "light_to_temperature";
                } else if row.starts_with("temperature-to-humidity map:") {
                    current_map = "temperature_to_humidity";
                } else if row.starts_with("humidity-to-location map:") {
                    current_map = "humidity_to_location";
                } else {
                    let numbers: Vec<u64> = row
                        .split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    match current_map {
                        "seed_to_soil" => seed_to_soil_map.push(numbers),
                        "soil_to_fertilizer" => soil_to_fertilizer_map.push(numbers),
                        "fertilizer_to_water" => fertilizer_to_water_map.push(numbers),
                        "water_to_light" => water_to_light_map.push(numbers),
                        "light_to_temperature" => light_to_temperature_map.push(numbers),
                        "temperature_to_humidity" => temperature_to_humidity_map.push(numbers),
                        "humidity_to_location" => humidity_to_location_map.push(numbers),
                        _ => (),
                    }
                }
            }
        }
    }

    let mut seed_to_soil_map = construct_map(&seed_to_soil_map);
    let mut soil_to_fertilizer_map = construct_map(&soil_to_fertilizer_map);
    let mut fertilizer_to_water_map = construct_map(&fertilizer_to_water_map);
    let mut water_to_light_map = construct_map(&water_to_light_map);
    let mut light_to_temperature_map = construct_map(&light_to_temperature_map);
    let mut temperature_to_humidity_map = construct_map(&temperature_to_humidity_map);
    let mut humidity_to_location_map = construct_map(&humidity_to_location_map);

    let mut _min_seed = 0;
    let mut _min_location = 0;

    for pair in (0..seeds.len()).step_by(2) {
        println!("pair: {}", pair);
        println!("seeds: {} {}", seeds[pair], seeds[pair + 1]);

        for seed in seeds[pair]..=seeds[pair] + seeds[pair + 1] {
            let mut soil = lookup_map(&seed_to_soil_map, seed);
            let mut fertilizer = lookup_map(&soil_to_fertilizer_map, soil);
            let mut water = lookup_map(&fertilizer_to_water_map, fertilizer);
            let mut light = lookup_map(&water_to_light_map, water);
            let mut temperature = lookup_map(&light_to_temperature_map, light);
            let mut humidity = lookup_map(&temperature_to_humidity_map, temperature);
            let mut location = lookup_map(&humidity_to_location_map, humidity);

            if _min_location == 0 || location < _min_location {
                _min_location = location;
                _min_seed = seed;
            }
        }
    }

    println!("min seed: {}", _min_seed);
    println!("min location: {}", _min_location);
}
