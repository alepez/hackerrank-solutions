use std::io::StdinLock;

struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}
struct Sticks(Vec<usize>);

fn solve(sticks: Sticks) -> Option<Triangle> {
    let mut v = sticks.0;

    // Sort in descending order (in place)
    v.sort_by(|a, b| b.cmp(a));

    v.iter()
        .zip(v.iter().skip(1))
        .zip(v.iter().skip(2))
        .map(|((&a, &b), &c)| Triangle { a, b, c })
        .find(|t| t.b + t.c > t.a)
}

impl std::fmt::Display for Triangle {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{} {} {}", self.c, self.b, self.a)
    }
}

impl From<StdinLock<'_>> for Sticks {
    fn from(stdin: StdinLock) -> Self {
        use std::io::BufRead;

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
}

fn main() {
    let sticks = std::io::stdin().lock().into();

    if let Some(triangle) = solve(sticks) {
        print!("{}", triangle);
    } else {
        print!("-1");
    }
}
