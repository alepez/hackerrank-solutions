use std::cmp::max;
use std::io::BufRead;

fn strong_password(password: &str) -> usize {
    let mut has_digit = 0;
    let mut has_lowercase = 0;
    let mut has_uppercase = 0;
    let mut has_special = 0;

    for c in password.chars() {
        match c {
            '0'..='9' => has_digit = 1,
            'a'..='z' => has_lowercase = 1,
            'A'..='Z' => has_uppercase = 1,
            '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' | '-' | '+' => {
                has_special = 1
            }
            _ => (),
        }
    }

    max(
        6 - (password.len() as i32),
        4 - has_digit - has_lowercase - has_uppercase - has_special,
    ) as usize
}

fn main() {
    let stdin = std::io::stdin();

    let line = stdin.lock().lines().skip(1).next().unwrap().unwrap();

    let result = strong_password(line.as_str());
    println!("{}", result);
}
