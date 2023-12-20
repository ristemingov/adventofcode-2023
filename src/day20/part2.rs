use num::Integer;
use std::collections::HashMap;

use crate::utils;

#[derive(Debug)]
struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }
    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[derive(Debug)]
struct Module {
    inst_kind: String,
    inst_name: String,
    outputs: Vec<String>,
    memory: HashMap<String, String>,
    ff_memory: String,
}

impl Module {
    fn set_ff_memory(&mut self, ff_memory: &String) {
        self.ff_memory = ff_memory.clone();
    }

    fn get_ff_memory(&self) -> String {
        self.ff_memory.clone()
    }
    fn get_outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }
    fn get_memory(&self) -> HashMap<String, String> {
        self.memory.clone()
    }
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut instructions: HashMap<String, Module> = HashMap::new();
    let mut broadcast_outputs: Vec<String> = Vec::new();

    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                let split_inst: Vec<String> = _row
                    .split(" -> ")
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>();
                let outputs = split_inst[1]
                    .split(", ")
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>();
                if split_inst[0] == "broadcaster" {
                    broadcast_outputs.extend(outputs.into_iter().clone());
                } else {
                    let inst_kind = split_inst[0][..1].to_string();
                    let inst_name = split_inst[0][1..].to_string();
                    let memory: HashMap<String, String> = HashMap::new();
                    let mut ffmemory = "".to_string();
                    if inst_kind == "%" {
                        ffmemory = "off".to_string();
                    }
                    let inst_module = Module {
                        outputs: outputs.clone(),
                        inst_kind: inst_kind,
                        inst_name: inst_name.clone(),
                        memory: memory.clone(),
                        ff_memory: ffmemory.clone(),
                    };
                    instructions.insert(inst_name, inst_module);
                }
            }
        }
    }

    let mut updates = Vec::new();

    for (name, module) in instructions.iter() {
        for output in &module.outputs {
            if let Some(out_module) = instructions.get(output) {
                if out_module.inst_kind == "&" {
                    updates.push((output.clone(), name.clone()));
                }
            }
        }
    }

    // Second pass: apply the updates
    for (output, name) in updates {
        if let Some(out_module) = instructions.get_mut(&output) {
            out_module.memory.insert(name, "lo".to_string());
        }
    }

    let mut feed: String = String::from("");
    let mut seen: HashMap<String, i32> = HashMap::new();
    let mut cycle_lengths: HashMap<String, i32> = HashMap::new();
    for (k, v) in &instructions {
        if v.outputs.contains(&"rx".to_string()) {
            feed = k.clone();
        }
    }

    for (k, v) in &instructions {
        if v.outputs.contains(&feed) {
            seen.insert(k.clone(), 0);
        }
    }

    let mut presses = 0;

    'mainLoop: loop {
        presses += 1;
        let mut queue: Queue<(String, String, String)> = Queue::new();

        for b in &broadcast_outputs {
            queue.enqueue(("broadcaster".to_string(), b.to_string(), "low".to_string()));
        }

        while !queue.is_empty() {
            let (origin, target, pulse) = queue.dequeue();

            if !instructions.contains_key(&target) {
                continue;
            }
            let module = instructions.get_mut(&target).unwrap();

            if module.inst_name == feed && pulse == "high" {
                let mut seen_num = *seen.get_mut(&origin).unwrap();
                seen_num += 1;
                seen.insert(origin.clone(), seen_num.clone());

                if !cycle_lengths.contains_key(&origin) {
                    cycle_lengths.insert(origin.clone(), presses);
                } else {
                    if let (Some(seen_value), Some(cycle_length)) =
                        (seen.get(&origin), cycle_lengths.get(&origin))
                    {
                        assert_eq!(presses, seen_value * cycle_length);
                    }
                }

                if seen.values().all(|&x| x != 0) {
                    let mut x: i128 = 1;
                    for &cl in cycle_lengths.values() {
                        x = x.lcm(&(cl as i128));
                    }
                    println!("{}", x);
                    break 'mainLoop;
                }
            }

            if module.inst_kind == "%" {
                if pulse == "low" {
                    let mut _out = "low".to_string();
                    if module.get_ff_memory() == "off" {
                        module.set_ff_memory(&"on".to_string());
                    } else {
                        module.set_ff_memory(&"off".to_string());
                    }
                    if module.get_ff_memory() == "on" {
                        _out = "high".to_string();
                    } else {
                        _out = "low".to_string();
                    }
                    for x in module.get_outputs() {
                        queue.enqueue((module.inst_name.clone(), x.clone(), _out.clone()));
                    }
                }
            } else {
                module.memory.insert(origin.clone(), pulse.clone());
                let mut _out1 = "high".to_string();

                if module.get_memory().values().all(|f| f == "high") {
                    _out1 = "low".to_string();
                } else {
                    _out1 = "high".to_string();
                }
                for x in module.get_outputs() {
                    queue.enqueue((module.inst_name.clone(), x.clone(), _out1.clone()));
                }
            }
        }
    }
}
