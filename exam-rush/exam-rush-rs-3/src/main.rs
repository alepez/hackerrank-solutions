fn solve(t: usize, mut v: Vec<usize>) -> usize {
    if t < v.len() {
        // Sets at the t-th position the element what would be there if the
        // array was sorted.
        // Puts before the t-th position all elements less than v[t]
        let (less_than_index, _, _) =
            v.select_nth_unstable(t);

        // Sort the first t elements
        less_than_index.sort();
    } else {
        // Sort everything when t >= n
        v.sort();
    }

    // Cast to isize is needed for subtraction
    let mut hours_left = t as isize;

    v.into_iter()
        .take_while(|&x| {
            hours_left -= x as isize;
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
