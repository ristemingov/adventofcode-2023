use crate::utils;

#[allow(dead_code)]
struct Hailstone {
    sx: i128,
    sy: i128,
    sz: i128,
    vx: i128,
    vy: i128,
    vz: i128,
    a: i128,
    b: i128,
    c: i128,
}

#[allow(dead_code)]
impl Hailstone {
    fn new(sx: i128, sy: i128, sz: i128, vx: i128, vy: i128, vz: i128) -> Self {
        let a = vy;
        let b = -vx;
        let c = vy * sx - vx * sy;
        Hailstone {
            sx,
            sy,
            sz,
            vx,
            vy,
            vz,
            a,
            b,
            c,
        }
    }

    fn to_string(&self) -> String {
        format!("Hailstone{{a={}, b={}, c={}}}", self.a, self.b, self.c)
    }
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut hailstones: Vec<Hailstone> = Vec::new();

    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                let parts: Vec<i128> = _row
                    .replace("@", ",")
                    .split(", ")
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();

                if parts.len() == 6 {
                    let hailstone =
                        Hailstone::new(parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]);
                    hailstones.push(hailstone);
                }
            }
        }
    }

    let mut total = 0;

    for (i, hs1) in hailstones.iter().enumerate() {
        for hs2 in &hailstones[..i] {
            let (a1, b1, c1) = (hs1.a, hs1.b, hs1.c);
            let (a2, b2, c2) = (hs2.a, hs2.b, hs2.c);
            if a1 * b2 == b1 * a2 {
                continue;
            }
            let denominator = a1 * b2 - a2 * b1;
            let x = (c1 * b2 - c2 * b1) as f64 / denominator as f64;
            let y = (c2 * a1 - c1 * a2) as f64 / denominator as f64;
            if (200000000000000.0..=400000000000000.0).contains(&x)
                && (200000000000000.0..=400000000000000.0).contains(&y)
            {
                if [(hs1, x, y), (hs2, x, y)].iter().all(|(hs, x, y)| {
                    (*x - hs.sx as f64) * hs.vx as f64 >= 0.0
                        && (*y - hs.sy as f64) * hs.vy as f64 >= 0.0
                }) {
                    total += 1;
                }
            }
        }
    }

    println!("{}", total);
}
