fn solve(c: i32, p: i32) -> Option<i32> {
    let diff = p - c;
    [20, 50, 100, 200, 500, 1000]
        .iter()
        .cloned()
        .filter(|&denom| diff <= denom)
        .find_map(|denom| {
            (0..=9).find_map(|a| {
                if diff + a == denom {
                    Some(a)
                } else {
                    None
                }
            })
        })
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let mut lines =
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

