use std::cmp::max;
use std::io::BufRead;

fn strong_password(password: &str) -> usize {
    let families = [
        '0'..='9',
        'a'..='A',
        'A'..='Z',
    ];

    let missing_families = password
        .chars()
        .fold([1, 1, 1, 1], |[dig, low, upp, spe], c| match c {
            '0'..='9' => [dig & 0, low, upp, spe],
            'a'..='z' => [dig, low & 0, upp, spe],
            'A'..='Z' => [dig, low, upp & 0, spe],
            _ => [dig, low, upp, spe & 0],
        })
        .iter()
        .sum::<i32>();

    max(6 - (password.len() as i32), missing_families) as usize
}

fn main() {
    let stdin = std::io::stdin();

    let line = stdin.lock().lines().skip(1).next().unwrap().unwrap();

    let result = strong_password(line.as_str());
    println!("{}", result);
}
