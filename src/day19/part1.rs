use crate::utils;

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

fn evaluate_part(rule: &Vec<String>, part: &Vec<(String, i32)>) -> String {
    for rule_counter in 0..rule.len() {
        for p in part {
            let rule_part = rule.get(rule_counter).unwrap();
            let (part_check, operator, value, destination) =
                get_rule_instructions(rule_part.to_string());
            if operator == "" {
                return destination;
            }

            if p.0 == part_check {
                if operator == ">" {
                    if p.1 > value {
                        return destination;
                    }
                } else {
                    if p.1 < value {
                        return destination;
                    }
                }
            }
        }
    }
    return "R".to_string();
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut parts: Vec<Vec<(String, i32)>> = Vec::new();
    let mut rules_map: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
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

    let mut _sum = 0;

    for part in parts {
        let mut start_rule_key = "in";
        let mut _evaluation = "".to_string();
        loop {
            _evaluation = evaluate_part(rules_map.get(start_rule_key).unwrap(), &part);
            if _evaluation == "R" {
                break;
            }
            if _evaluation == "A" {
                for p in &part {
                    _sum += p.1;
                }
                break;
            }
            start_rule_key = &_evaluation;
        }
    }
    println!("{:?}", _sum);
}
