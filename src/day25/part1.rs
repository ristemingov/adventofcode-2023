use crate::utils;

use std::collections::{HashMap, VecDeque};

struct Graph {
    g: HashMap<String, HashMap<String, i32>>,
    parent: HashMap<String, String>,
}

impl Graph {
    fn new(g: HashMap<String, HashMap<String, i32>>) -> Self {
        let parent = g.keys().map(|k| (k.clone(), String::new())).collect();
        Graph { g, parent }
    }

    fn bfs(&mut self, s: &str, t: &str) -> bool {
        for key in self.g.keys() {
            self.parent.insert(key.clone(), String::new());
        }
        self.parent.insert(s.to_string(), s.to_string());

        let mut q = VecDeque::new();
        q.push_back(s.to_string());

        while let Some(n) = q.pop_front() {
            for (e, &c) in &self.g[&n] {
                if c > 0 && self.parent[e].is_empty() {
                    self.parent.insert(e.clone(), n.clone());
                    q.push_back(e.clone());
                }
            }
        }

        !self.parent[t].is_empty()
    }

    fn min_cut(&mut self, s: &str, t: &str) -> i32 {
        for (_v, e) in self.g.iter_mut() {
            for k in e.keys().cloned().collect::<Vec<_>>() {
                e.insert(k, 1);
            }
        }

        let mut max_flow = 0;
        while self.bfs(s, t) {
            let mut flow = i32::MAX;
            let mut n = t.to_string();

            while n != s {
                flow = flow.min(self.g[&self.parent[&n]][&n]);
                n = self.parent[&n].clone();
            }

            max_flow += flow;
            let mut v = t.to_string();

            while v != s {
                let u = self.parent[&v].clone();
                *self.g.get_mut(&u).unwrap().get_mut(&v).unwrap() -= flow;
                *self.g.get_mut(&v).unwrap().entry(u.clone()).or_insert(0) += flow;
                v = u;
            }
        }

        max_flow
    }

    fn solve(&mut self) -> i32 {
        let g1 = self.parent.iter().filter(|&(_, p)| !p.is_empty()).count() as i32;
        (self.g.len() as i32 - g1) * g1
    }
}



pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut g = HashMap::new();
    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                let parts: Vec<_> = _row.split(": ").collect();
                let (lhs, rhs) = (parts[0], parts[1]);
                for r in rhs.split_whitespace() {
                    g.entry(lhs.to_string()).or_insert_with(HashMap::new).insert(r.to_string(), 1);
                    g.entry(r.to_string()).or_insert_with(HashMap::new).insert(lhs.to_string(), 1);
                }
            }
        }
    }

    let mut graph = Graph::new(g);
    let (s, other) = {
        let mut nodes = graph.g.keys().cloned().collect::<Vec<_>>();
        (nodes.remove(0), nodes)
    };

    for t in other {
        if graph.min_cut(&s, &t) == 3 {
            break;
        }
    }

    println!("{}", graph.solve());
}
