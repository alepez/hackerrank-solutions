use std::collections::BTreeMap;

fn count_frequencies(string: &str) -> BTreeMap<u8, i32> {
    let mut f = BTreeMap::new();

    for c in b'a'..=b'z' {
        f.insert(c, 0);
    }

    for c in string.as_bytes() {
        let v = f.get_mut(c);
        match v {
            Some(v) => *v = *v + 1,
            None => (),
        }
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
    let mut diff = BTreeMap::new();

    for c in b'a'..=b'z' {
        let d = left_freq.get(&c).unwrap() - right_freq.get(&c).unwrap();
        diff.insert(c, d.abs());
    }

    let sum = diff.values().fold(0i32, |sum, val| sum + val);

    sum / 2
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
