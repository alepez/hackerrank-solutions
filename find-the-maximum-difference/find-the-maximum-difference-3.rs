fn solve(arr: Vec<u32>) -> u32 {
    arr.into_iter()
        .fold((std::u32::MAX, 0), |(min, max_diff), x| {
            if x < min {
                (x, max_diff)
            } else {
                (min, std::cmp::max(max_diff, x - min))
            }
        })
        .1
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let numbers = stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .skip(1)
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|x| x.parse().ok())
        .collect();

    let output = solve(numbers);
    println!("{}", output);
}
