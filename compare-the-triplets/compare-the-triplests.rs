use std::cmp::Ordering;

fn compare_triplets(a: &[u32], b: &[u32]) -> (u32, u32) {
    a.iter()
        .zip(b.iter())
        .fold((0, 0), |(ap, bp), (ai, bi)| match ai.cmp(bi) {
            Ordering::Less => (ap, bp + 1),
            Ordering::Equal => (ap, bp),
            Ordering::Greater => (ap + 1, bp),
        })
}

fn parse_line<T>(lines: &mut T) -> Vec<u32>
where
    T: Iterator<Item = String>,
{
    lines
        .next()
        .unwrap()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let a = parse_line(&mut lines);
    let b = parse_line(&mut lines);

    let (a, b) = compare_triplets(&a, &b);

    println!("{} {}", a, b);
}
