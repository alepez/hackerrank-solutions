fn solve(numbers: Vec<i32>, queries: Vec<(usize, usize)>) {
    let sums: Vec<_> = numbers
        .iter()
        .scan(0i32, |acc, &n| {
            *acc = (*acc + n).abs();
            Some(*acc)
        })
        .collect();

    for (from, to) in queries {
        let l = if from == 1 { 0 } else { sums[from - 2] };
        let r = sums[to - 1];
        let is_odd = (r - l) & 1 == 1;
        println!("{}", if is_odd { "Odd" } else { "Even" });
    }
}

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

    solve(numbers, queries);
}
