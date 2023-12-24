use crate::utils;

#[allow(dead_code)]
struct Hailstone {
    sx: i64,
    sy: i64,
    sz: i64,
    vx: i64,
    vy: i64,
    vz: i64,
    a: i64,
    b: i64,
    c: i64,
}

impl Hailstone {
    fn new(sx: i64, sy: i64, sz: i64, vx: i64, vy: i64, vz: i64) -> Self {
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
}

fn matrix_transpose(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
    (0..m[0].len())
        .map(|i| m.iter().map(|row| row[i]).collect::<Vec<_>>())
        .collect()
}

fn matrix_minor(m: &[Vec<f64>], i: usize, j: usize) -> Vec<Vec<f64>> {
    m.iter()
        .enumerate()
        .filter(|&(r, _)| r != i)
        .map(|(_, row)| {
            row.iter()
                .enumerate()
                .filter(|&(c, _)| c != j)
                .map(|(_, &item)| item)
                .collect()
        })
        .collect()
}

fn matrix_det(m: &[Vec<f64>]) -> f64 {
    if m.len() == 2 {
        return m[0][0] * m[1][1] - m[0][1] * m[1][0];
    }

    let mut determinant = 0.0;
    for c in 0..m.len() {
        determinant += (-1.0f64).powi(c as i32) * m[0][c] * matrix_det(&matrix_minor(m, 0, c));
    }

    determinant
}

fn matrix_inverse(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
    // needs to be revisited
    let determinant = matrix_det(m);
    let mut cofactors = vec![vec![0.0; m.len()]; m.len()];

    for r in 0..m.len() {
        for c in 0..m.len() {
            let minor = matrix_minor(m, r, c);
            cofactors[r][c] = (-1.0f64).powi((r + c) as i32) * matrix_det(&minor);
        }
    }

    let cofactors_transposed = matrix_transpose(&cofactors);

    cofactors_transposed
        .iter()
        .map(|row| row.iter().map(|&item| item / determinant).collect())
        .collect()
}

fn vector_diff(a: &[f64], b: &[f64]) -> Vec<f64> {
    a.iter().zip(b.iter()).map(|(&ai, &bi)| ai - bi).collect()
}

fn get_equations(a: &[f64], va: &[f64], b: &[f64], vb: &[f64]) -> (Vec<Vec<f64>>, Vec<f64>) {
    let diff = vector_diff(a, b);
    let dvel = vector_diff(va, vb);

    let a_matrix = vec![
        vec![0.0, -dvel[2], dvel[1], 0.0, -diff[2], diff[1]],
        vec![dvel[2], 0.0, -dvel[0], diff[2], 0.0, -diff[0]],
        vec![-dvel[1], dvel[0], 0.0, -diff[1], diff[0], 0.0],
    ];

    let b_vector = vec![
        b[1] * vb[2] - b[2] * vb[1] - (a[1] * va[2] - a[2] * va[1]),
        b[2] * vb[0] - b[0] * vb[2] - (a[2] * va[0] - a[0] * va[2]),
        b[0] * vb[1] - b[1] * vb[0] - (a[0] * va[1] - a[1] * va[0]),
    ];

    (a_matrix, b_vector)
}

fn matrix_mul(m: &[Vec<f64>], vec: &[f64]) -> Vec<f64> {
    m.iter()
        .map(|row| row.iter().zip(vec.iter()).map(|(&r, &v)| r * v).sum())
        .collect()
}

fn find_coord_sum(hailstones: &Vec<Hailstone>) -> f64 {
    let (a, va) = (
        vec![
            hailstones[0].sx as f64,
            hailstones[0].sy as f64,
            hailstones[0].sz as f64,
        ],
        vec![
            hailstones[0].vx as f64,
            hailstones[0].vy as f64,
            hailstones[0].vz as f64,
        ],
    );
    let (b, vb) = (
        vec![
            hailstones[1].sx as f64,
            hailstones[1].sy as f64,
            hailstones[1].sz as f64,
        ],
        vec![
            hailstones[1].vx as f64,
            hailstones[1].vy as f64,
            hailstones[1].vz as f64,
        ],
    );
    let (c, vc) = (
        vec![
            hailstones[2].sx as f64,
            hailstones[2].sy as f64,
            hailstones[2].sz as f64,
        ],
        vec![
            hailstones[2].vx as f64,
            hailstones[2].vy as f64,
            hailstones[2].vz as f64,
        ],
    );

    let (a1, b1) = get_equations(&a, &va, &b, &vb);
    let (a2, b2) = get_equations(&a, &va, &c, &vc);

    let a = [a1, a2].concat();
    let b = [b1, b2].concat();

    let x = matrix_mul(&matrix_inverse(&a), &b);
    x.iter().take(3).map(|&num| num.round()).sum()
}

pub fn solve(file_path: &str) {
    println!("In file {}", file_path);
    let mut hailstones: Vec<Hailstone> = Vec::new();

    if let Ok(lines) = utils::misc::read_lines(file_path) {
        for (_i, _line) in lines.enumerate() {
            if let Ok(_row) = _line {
                let parts: Vec<i64> = _row
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

    let sum = find_coord_sum(&hailstones);
    println!("{}", sum);
}
