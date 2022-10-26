use core::cmp::{max, min};
use std::io::{self, BufRead};

fn euclidean(x: i64, y: i64) -> f64 {
    (((x * x) + (y * y)) as f64).sqrt()
}

fn parse_point(s: String) -> Option<(i64, i64)> {
    let (x, y) = s.split_once(' ')?;
    let x: i64 = x.parse().ok()?;
    let y: i64 = y.parse().ok()?;
    Some((x, y))
}

fn solve<T: BufRead>(lines: io::Lines<T>) -> f64 {
    let mut min_ver = 10_000_000_000;
    let mut max_ver = -10_000_000_000;
    let mut min_hor = 10_000_000_000;
    let mut max_hor = -10_000_000_000;

    lines
        .filter_map(|r| r.ok())
        .filter_map(parse_point)
        .for_each(|(x, y)| {
            if x == 0 {
                min_ver = min(min_ver, y);
                max_ver = max(max_ver, y);
            } else if y == 0 {
                min_hor = min(min_hor, x);
                max_hor = max(max_hor, x);
            }
        });

    let distances = [
        (max_hor - min_hor) as f64,
        (max_ver - min_ver) as f64,
        euclidean(min_hor, max_ver),
        euclidean(min_hor, min_ver),
        euclidean(max_hor, max_ver),
        euclidean(min_hor, min_ver),
    ];

    *distances
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let _ = lines.next();
    println!("{}", solve(lines));
}
