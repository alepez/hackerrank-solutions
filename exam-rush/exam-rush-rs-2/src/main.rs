fn solve(
    mut hours_left: isize,
    mut tm: Vec<isize>,
) -> usize {
    tm.sort();
    tm.into_iter()
        .take_while(|&x| {
            hours_left -= x;
            hours_left >= 0
        })
        .count()
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut lines = stdin.lines();

    let hours_left = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let tm = lines
        .map(|x| x.unwrap().parse().unwrap())
        .collect();

    let max_num_chapters = solve(hours_left, tm);

    println!("{}", max_num_chapters);
}
