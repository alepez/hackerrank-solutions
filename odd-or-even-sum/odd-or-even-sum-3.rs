#[derive(Debug)]
enum Parity {
    Odd,
    Even,
}

impl From<i32> for Parity {
    fn from(n: i32) -> Self {
        if n & 1 == 1 {
            Parity::Odd
        } else {
            Parity::Even
        }
    }
}

fn solve<N, Q>(numbers: N, queries: Q) -> impl Iterator<Item = Parity>
where
    N: IntoIterator<Item = i32>,
    Q: IntoIterator<Item = (usize, usize)>,
{
    let sums: Vec<_> = numbers
        .into_iter()
        .scan(0i32, |acc, n| {
            *acc = (*acc + n).abs();
            Some(*acc)
        })
        .collect();

    queries.into_iter().map(move |(from, to)| {
        let l = if from == 1 { 0 } else { sums[from - 2] };
        let r = sums[to - 1];
        Parity::from(r - l)
    })
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

    let queries = lines.map(|query_str| {
        let q: Vec<usize> = query_str.split(' ').map(|x| x.parse().unwrap()).collect();
        (q[0], q[1])
    });

    solve(numbers, queries).for_each(|v| println!("{:?}", v));
}
