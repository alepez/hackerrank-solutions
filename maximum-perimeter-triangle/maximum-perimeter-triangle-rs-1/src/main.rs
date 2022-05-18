pub struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}
pub struct Sticks(Vec<usize>);

pub fn solve(sticks: Sticks) -> Option<Triangle> {
    let mut v = sticks.0;

    // Sort in descending order (in place)
    v.sort_by(|a, b| b.cmp(a));

    v.iter()
        .zip(v.iter().skip(1))
        .zip(v.iter().skip(2))
        .map(|((&a, &b), &c)| Triangle { a, b, c })
        .find(|t| t.b + t.c > t.a)
}

fn read_input() -> Sticks {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let lines = stdin.lines();

    let v = lines
        .skip(1)
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    Sticks(v)
}

fn print_output(triangle: Triangle) {
    print!("{} {} {}", triangle.c, triangle.b, triangle.a);
}

fn main() {
    let sticks = read_input();

    if let Some(triangle) = solve(sticks) {
        print_output(triangle);
    } else {
        print!("-1");
    }
}
