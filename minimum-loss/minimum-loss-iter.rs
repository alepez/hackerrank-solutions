use std::cmp::min;
use std::collections::BTreeSet;
use std::error::Error;
use std::io::BufRead;

fn minimum_loss<T: IntoIterator<Item = u64>>(prices: T) -> u64 {
    let mut set = BTreeSet::new();

    let mut min_loss = std::u64::MAX;

    for price in prices {
        let lb = set.range(price..).next().cloned();
        set.insert(price);
        if let Some(lb) = lb {
            min_loss = min(min_loss, lb - price);
        }
    }

    min_loss
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
