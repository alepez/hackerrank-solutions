use std::{fmt::Debug, iter::FromIterator, str::FromStr};

fn next_parsed_line<C, T, L>(lines: &mut L) -> C
where
    L: Iterator<Item = String>,
    C: FromIterator<T>,
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    lines
        .next()
        .unwrap()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn angry_professor(k: usize, a: &[i32]) -> bool {
    k !=  a.iter().filter(|x| **x <= 0).take(k).count()
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let nk: Vec<usize> = next_parsed_line(&mut lines);
        let k = nk[1];
        let vec: Vec<i32> = next_parsed_line(&mut lines);
        let is_angry = angry_professor(k, vec.as_slice());
        println!("{}", if is_angry { "YES" } else { "NO" });
    }
}
