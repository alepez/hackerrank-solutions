fn solve(arr: Vec<u32>) -> u32 {
    let mut arr: Vec<(usize, u32)> =
        arr.into_iter().enumerate().collect();
    arr.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut asc = arr.iter();
    let mut desc = arr.iter().rev();
    let mut max_diff = 0;

    let mut l = asc.next();
    let mut r = desc.next();

    while let Some((li, lv)) = l {
        if let Some((ri, rv)) = r {
            if li < ri && lv < rv {
                max_diff = rv - lv;
                break;
            } else {
                r = desc.next();
            }
        } else {
            l = asc.next();
            desc = arr.iter().rev();
            r = desc.next();
        }
    }

    max_diff
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let numbers = stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .skip(1)
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|x| x.parse().ok())
        .collect();

    let output = solve(numbers);
    println!("{}", output);
}
