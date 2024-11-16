use std::{
    cmp::{max, min, Ordering},
    io::BufRead as _,
};

fn solve(m: usize, flavors: Vec<usize>) -> (usize, usize) {
    let mut flavors: Vec<_> = flavors.into_iter().enumerate().collect();
    flavors.sort_by_key(|f| f.1);

    let mut left_it = 0;
    let mut right_it = flavors.len() - 1;

    loop {
        let (left_index, left_val) = flavors[left_it];
        let (right_index, right_val) = flavors[right_it];
        let sum = left_val + right_val;

        match sum.cmp(&m) {
            Ordering::Less => {
                left_it += 1;
            }
            Ordering::Equal => {
                return (
                    1 + min(left_index, right_index),
                    1 + max(left_index, right_index),
                )
            }
            Ordering::Greater => {
                right_it -= 1;
            }
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let m = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
        let _n = lines.next();
        let flavors: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let (alice, bob) = solve(m, flavors);
        println!("{} {}", alice, bob);
    }
}
