fn reduce_string(s: String) -> String {
    let reduced = s.chars().fold(String::new(), |mut acc, c| {
        match acc.pop() {
            None => acc.push(c),
            Some(l) => {
                if l != c {
                    acc.push(l);
                    acc.push(c);
                }
            }
        };

        acc
    });

    if reduced != s {
        reduce_string(reduced)
    } else {
        reduced
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let s = lines.next().unwrap();
    let result = reduce_string(s);
    if result.is_empty() {
        println!("Empty String");
    } else {
        println!("{}", result);
    }
}
