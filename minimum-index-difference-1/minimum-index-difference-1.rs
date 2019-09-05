fn parse_line(line: &str) -> Vec<i32> {
    line.split(' ').map(|x| x.parse().unwrap()).collect()
}

fn enumerate_and_sort(vec: Vec<i32>) -> Vec<(i32, i32)> {
    let mut vec: Vec<(usize, &i32)> = vec.iter().enumerate().collect();
    vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    vec.iter().map(|x| (x.0 as i32, *x.1)).collect()
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let mut lines = stdin.lock().lines().map(Result::unwrap).skip(1);

    let one = enumerate_and_sort(parse_line(lines.next().unwrap().as_str()));
    let two = enumerate_and_sort(parse_line(lines.next().unwrap().as_str()));

    let sum: i32 = one
        .iter()
        .zip(two.iter())
        .map(|(x, y)| (x.0 - y.0).abs())
        .sum();

    println!("{:?}", sum / 2);
}
