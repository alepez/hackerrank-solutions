// This solutions passes all tests on HackerRank, even if the algorithm is NxQ
fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let mut lines = stdin.lock().lines().map(Result::unwrap).skip(1);

    let numbers: Vec<i32> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let queries: Vec<(usize, usize)> = lines
        .map(|query_str| {
            let q: Vec<usize> = query_str.split(' ').map(|x| x.parse().unwrap()).collect();
            (q[0], q[1])
        })
        .collect();

    for (from, to) in queries {
        let slice = &numbers[from-1..to];
        let x : i32 = slice.iter().fold(0, |acc, n| acc.abs() + n);
        println!("{}", if x & 1 == 1 { "Odd" } else { "Even" });
    }
}
