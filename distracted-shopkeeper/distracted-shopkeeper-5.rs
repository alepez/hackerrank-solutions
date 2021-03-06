fn solve(c: i32, p: i32) -> i32 {
    let diff = p - c;

    let mut result = -1;

    for &denom in &[20, 50, 100, 200, 500, 1000] {
        if denom > diff + 9 {
            break;
        }

        if diff <= denom {
            result = denom - diff;
            break;
        }
    }

    result
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .skip(1)
        .for_each(|line| {
            let numbers: Vec<i32> = line
                .split(' ')
                .map(|x| x.parse().unwrap())
                .collect();
            let result = solve(numbers[0], numbers[1]);
            println!("{}", result);
        });
}
