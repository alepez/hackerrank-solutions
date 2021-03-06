fn solve(m: u32, k: u32, btc_to_i: &[u32], i_to_usd: &[u32]) -> u32 {
    use std::cmp::max;

    let best_buy_sell = (btc_to_i.iter().zip(i_to_usd))
        .map(|(bi, iu)| bi * iu)
        .max()
        .unwrap();

    m * max(k, best_buy_sell)
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();

    let lines: Vec<Vec<u32>> = stdin
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .flat_map(str::parse)
                .collect()
        })
        .collect();

    let m = lines[0][1];
    let k = lines[0][2];

    let btc_to_i = lines[1].as_slice();
    let i_to_usd = lines[2].as_slice();

    println!("{:?}", solve(m, k, btc_to_i, i_to_usd));
}
