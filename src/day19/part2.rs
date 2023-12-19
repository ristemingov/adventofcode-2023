use crate::utils;
use std::collections::HashMap;

fn get_rule_instructions(rule: String) -> (String, String, i32, String) {
    let mut _destination = "".to_string();
    let mut _operator = "".to_string();
    let mut _value = 0;
    let mut _part_check = "".to_string();

    let rule_parts: Vec<&str> = rule.split(":").collect();
    if rule_parts.len() == 1 {
        _destination = rule_parts[0].to_string();
        return (_part_check, _operator, _value, _destination);
    }
    let rule_part = rule_parts[0];
    if rule_part.contains("<") {
        let mut rule_parts_op = rule_part.split("<");
        _part_check = rule_parts_op.next().unwrap().to_string();
        _operator = "<".to_string();
        _value = rule_parts_op.next().unwrap().parse::<i32>().unwrap();
        _destination = rule_parts[1].to_string();
    } else if rule_part.contains(">") {
        let mut rule_parts_op = rule_part.split(">");
        _part_check = rule_parts_op.next().unwrap().to_string();
        _operator = ">".to_string();
        _value = rule_parts_op.next().unwrap().parse::<i32>().unwrap();
        _destination = rule_parts[1].to_string();
    } else {
        _destination = rule_parts[1].to_string();
    }
    return (_part_check, _operator, _value, _destination);
}

fn parse_instructions(
    rules_map: &HashMap<String, Vec<String>>,
    target: &str,
) -> (Vec<(String, String, i32, String)>, String) {
    let mut rule_instructions = rules_map.get(target).unwrap().clone();

    let fallback = rule_instructions.pop().unwrap();

    let mut parsed_instructions = Vec::new();
    for rule in rule_instructions {
        parsed_instructions.push(get_rule_instructions(rule));
    }

    return (parsed_instructions, fallback);
}

fn count_combinations(
    rules_map: &HashMap<String, Vec<String>>,
    ranges: &mut HashMap<String, (i32, i32)>,
    name: &str,
) -> i128 {
    match name {
        "R" => 0,
        "A" => {
            let mut product: i128 = 1;
            for &(lo, hi) in ranges.values() {
                product *= hi as i128 - lo as i128 + 1;
            }
            product
        }
        _ => {
            let (rules, fallback) = parse_instructions(&rules_map, name);
            let mut total = 0;

            let mut did_not_break = true;
            for (key, cmp, n, ref target) in rules {
                let (lo, hi) = ranges[key.as_str()];

                let (t, f) = if cmp == "<" {
                    ((lo, n - 1), (n, hi))
                } else {
                    ((n + 1, hi), (lo, n))
                };

                if t.0 <= t.1 {
                    let mut copy = HashMap::new();
                    for (key, value) in &mut *ranges {
                        copy.insert(key.clone(), value.clone());
                    }
                    copy.insert(key.clone(), t.clone());
                    total += count_combinations(&rules_map, &mut copy, target);
                }
                if f.0 <= f.1 {
                    ranges.insert(key.clone(), f.clone());
                } else {
                    did_not_break = false;
                    break;
                }
            }
            if did_not_break {
                total += count_combinations(&rules_map, ranges, fallback.as_str());
            }

            total
        }
    }
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut parts: Vec<Vec<(String, i32)>> = Vec::new();
    let mut rules_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut save_parts: bool = false;

    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                if _row == "" {
                    save_parts = true;
                    continue;
                }

                if save_parts {
                    let parts_parsed = _row.replace("{", "").replace("}", "");
                    let parts_vec: Vec<String> =
                        parts_parsed.split(",").map(|s| s.to_string()).collect();
                    parts.push(
                        parts_vec
                            .iter()
                            .filter_map(|s| {
                                let parts: Vec<&str> = s.split('=').collect();
                                if parts.len() == 2 {
                                    let key = parts[0].to_string();
                                    let value = parts[1].parse::<i32>().ok()?;
                                    Some((key, value))
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    );
                } else {
                    let mut rule_parts = _row.split("{");
                    let rule_id = rule_parts.next().unwrap();
                    let rule_value_vec = rule_parts
                        .next()
                        .unwrap()
                        .replace("}", "")
                        .split(",")
                        .map(|s| s.to_string())
                        .collect();
                    rules_map.insert(rule_id.to_string(), rule_value_vec);
                }
            }
        }
    }
    let mut ranges: HashMap<String, (i32, i32)> = HashMap::from([
        ("x".to_string(), (1, 4000)),
        ("m".to_string(), (1, 4000)),
        ("a".to_string(), (1, 4000)),
        ("s".to_string(), (1, 4000)),
    ]);
    let mut _sum = count_combinations(&rules_map, &mut ranges, "in");
    println!("{:?}", _sum);
}
