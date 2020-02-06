fn solve(c: i32, p: i32) -> Option<i32> {
    let diff = p - c;

    let mut result = None;

    for &denom in &[20, 50, 100, 200, 500, 1000] {
        if diff <= denom {
            for a in 0..=9 {
                if diff + a == denom {
                    result = Some(a)
                }
            }
        }
    }

    result
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let lines =
        stdin.lock().lines().map(Result::unwrap).skip(1);

    lines.for_each(|line| {
        let numbers: Vec<i32> = line
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        let result =
            solve(numbers[0], numbers[1]).unwrap_or(-1);
        println!("{}", result);
    });
}

