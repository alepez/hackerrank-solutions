fn solve(
    mut hours_left: usize,
    mut tm: Vec<usize>,
) -> usize {
    tm.sort();

    let mut max_num_chapters = 0;

    for x in tm {
        if hours_left >= x {
            hours_left -= x;
            max_num_chapters += 1;
        } else {
            break;
        }
    }

    max_num_chapters
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
