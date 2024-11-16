use std::io::BufRead as _;

fn solve(s: &str, n: usize) -> usize {
    let chars_count = s.chars().count();
    let remainder = n % chars_count;
    let repeat = n / chars_count;
    let a_in_s = s.chars().filter(|&c| c == 'a').count();
    let a_in_remainder = s.chars().take(remainder).filter(|&c| c == 'a').count();
    repeat * a_in_s + a_in_remainder
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let s = lines.next().unwrap().unwrap();
    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    println!("{}", solve(&s, n));
}
