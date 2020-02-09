fn solve(arr: Vec<u32>) -> u32 {
    let mut it = arr.into_iter();
    let mut min = it.next().unwrap();
    let mut max = 0;

    it.for_each(|x| {
        if x < min {
            min = x;
        } else {
            max = std::cmp::max(max, x - min);
        }
    });

    max
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
