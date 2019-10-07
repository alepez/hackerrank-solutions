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

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let a: Vec<u32> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let b: Vec<u32> = lines
        .next()
        .unwrap()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let (a, b) = compare_triplets(&a, &b);

    println!("{} {}", a, b);
}
