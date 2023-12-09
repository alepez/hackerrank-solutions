extern crate the_master_of_cryptocurrencies;

use the_master_of_cryptocurrencies::*;

fn _read_from_stdin() -> (u32, u32, Vec<u32>, Vec<u32>) {
    use std::io::BufRead;

    let stdin = std::io::stdin();

    let mut lines: Vec<Vec<u32>> = stdin
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

    let i_to_usd = lines.pop().unwrap();
    let btc_to_i = lines.pop().unwrap();

    (m, k, btc_to_i, i_to_usd)
}

fn generate() -> (u32, u32, Vec<u32>, Vec<u32>) {
    let a = (1..=1_000).cycle().take(1_000_000);
    let b = (1..=1_000).rev().cycle().take(1_000_000);
    (8, 7, a.collect(), b.collect())
}

fn main() {
    let (m, k, btc_to_i, i_to_usd) = generate();

    println!("{:?}", solve_fold(m, k, &btc_to_i, &i_to_usd));
    println!("{:?}", solve_map_then_max(m, k, &btc_to_i, &i_to_usd));
    println!("{:?}", solve_parallel(m, k, &btc_to_i, &i_to_usd));
    println!("{:?}", solve_parallel_reduce(m, k, &btc_to_i, &i_to_usd));
    println!("{:?}", solve_parallel_mapreduce(m, k, &btc_to_i, &i_to_usd));
}

