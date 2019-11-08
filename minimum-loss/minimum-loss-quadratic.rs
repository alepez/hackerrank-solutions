use std::cmp::min;
use std::error::Error;
use std::io::{self, BufRead};

fn minimum_loss(prices: Vec<u64>) -> u64 {
    println!("{:?}", prices);

    let mut min_loss = std::u64::MAX;

    for i in prices.iter() {
        for j in prices.iter().skip(1) {
            if i > j {
                min_loss = min(min_loss, i - j);
            }
        }
    }

    min_loss
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let prices: Vec<u64> = stdin
        .lock()
        .lines()
        .skip(1)
        .next()
        .ok_or("Missing line")??
        .split(' ')
        .flat_map(str::parse)
        .collect();
    println!("{}", minimum_loss(prices));
    Ok(())
}
