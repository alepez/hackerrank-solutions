fn to_index(c: u8) -> usize {
    (c - b'a') as usize
}

fn count_frequencies(string: &str) -> Vec<i32> {
    let mut f = Vec::new();

    f.resize(to_index(b'z') + 1, 0);

    for c in string.as_bytes() {
        f[to_index(*c)] += 1;
    }

    f
}

fn count_changes(string: &str) -> i32 {
    if (string.len() & 1) == 1 {
        return -1;
    }

    let (left, right) = string.split_at(string.len() / 2);

    let left_freq = count_frequencies(left);
    let right_freq = count_frequencies(right);

    left_freq
        .into_iter()
        .zip(right_freq)
        .fold(0, |sum, (l, r)| sum + (l - r).abs())
        / 2
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let lines = stdin.lock().lines().map(Result::unwrap).skip(1);
    let results = lines.map(|string| count_changes(string.as_str()));

    for result in results {
        println!("{:}", result);
    }
}
