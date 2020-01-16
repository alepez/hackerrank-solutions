use std::io::Write;
use std::iter::once;

fn solve<N, Q>(numbers: N, queries: Q) -> impl Iterator<Item = bool>
where
    N: IntoIterator<Item = i32>,
    Q: IntoIterator<Item = (usize, usize)>,
{
    let mut acc = 0;

    let sums: Vec<_> = once(0)
        .chain(numbers.into_iter().map(|n| {
            acc = acc ^ n;
            acc
        }))
        .collect();

    queries.into_iter().map(move |(from, to)| {
        let l = sums[from - 1];
        let r = sums[to];
        (r ^ l) & 1 == 1
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

    solve(numbers, queries)
        .map(|is_odd| {
            if is_odd {
                b"Odd\n".as_ref()
            } else {
                b"Even\n".as_ref()
            }
        })
        .for_each(|v| {
            io::stdout().write_all(v).unwrap();
        });
}
