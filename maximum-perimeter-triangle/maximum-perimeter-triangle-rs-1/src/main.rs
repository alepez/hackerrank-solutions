struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}

struct Sticks(Vec<usize>);

fn solve(mut sticks: Sticks) -> Option<Triangle> {
    // Sort in descending order
    sticks.0.sort_by(|a, b| b.cmp(a));

    for i in 2..sticks.0.len() {
        let t = Triangle {
            a: sticks.0[i],
            b: sticks.0[i - 1],
            c: sticks.0[i - 2],
        };

        if t.b + t.c > t.a {
            return Some(t);
        }
    }

    None
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
    print!("{} {} {}", triangle.a, triangle.b, triangle.c);
}

fn main() {
    let sticks = read_input();

    if let Some(triangle) = solve(sticks) {
        print_output(triangle);
    } else {
        print!("-1");
    }
}
