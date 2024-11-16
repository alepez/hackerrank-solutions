use std::io::BufRead as _;

fn solve(m: usize, flavors: &[usize]) -> (usize, usize) {
    for (i, x) in flavors.iter().enumerate() {
        for (j, y) in flavors.iter().enumerate() {
            if ((x + y) == m) && (i != j) {
                return (i + 1, j + 1);
            }
        }
    }
    unreachable!()
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
        let (alice, bob) = solve(m, &flavors);
        println!("{} {}", alice, bob);
    }
}
