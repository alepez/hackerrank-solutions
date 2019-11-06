use std::error::Error;
use std::io::BufRead;
use std::iter::FromIterator;

fn minimum_loss<T>(prices: T) -> u64
where
    T: std::iter::Iterator<Item = u64>,
{
    let mut prices = Vec::from_iter(prices.enumerate());
    prices.sort_by(|(_, a), (_, b)| a.cmp(b));
    prices
        .iter()
        .zip(prices.iter().skip(1))
        .filter(|((sell_time, _), (buy_time, _))| buy_time < sell_time)
        .map(|(&(_, sell_price), &(_, buy_price))| buy_price - sell_price)
        .min()
        .unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();

    let line = stdin
        .lock()
        .lines()
        .skip(1)
        .next()
        .ok_or("Missing line")??;

    let prices = line.split_whitespace().flat_map(str::parse);

    let result = minimum_loss(prices);

    println!("{}", result);

    Ok(())
}
