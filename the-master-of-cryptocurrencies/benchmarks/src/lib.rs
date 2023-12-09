extern crate rayon;

pub fn solve_map_then_max(m: u32, k: u32, btc_to_i: &[u32], i_to_usd: &[u32]) -> u32 {
    use std::cmp::max;

    let best_buy_sell = (btc_to_i.iter().zip(i_to_usd))
        .map(|(bi, iu)| bi * iu)
        .max()
        .unwrap();

    m * max(k, best_buy_sell)
}

pub fn solve_fold(m: u32, k: u32, btc_to_i: &[u32], i_to_usd: &[u32]) -> u32 {
    use std::cmp::max;

    btc_to_i.iter().zip(i_to_usd)
        .fold(k, |best, (bi, iu)| max(best, bi * iu)) * m
}

pub fn solve_parallel(m: u32, k: u32, btc_to_i: &[u32], i_to_usd: &[u32]) -> u32 {
    use std::cmp::max;
    use rayon::prelude::*;

    let best_buy_sell = btc_to_i.par_iter().copied().zip(i_to_usd.par_iter().copied())
        .map(|(bi, iu)| bi * iu)
        .max().unwrap();

    m * max(k, best_buy_sell)
}

pub fn solve_parallel_reduce(m: u32, k: u32, btc_to_i: &[u32], i_to_usd: &[u32]) -> u32 {
    use std::cmp::max;
    use rayon::prelude::*;

    btc_to_i.par_iter().copied().zip(i_to_usd.par_iter().copied())
        .reduce(|| (k, 1), |(best, _), (bi, iu)| (max(best, bi * iu), 1)).0 * m
}

pub fn solve_parallel_mapreduce(m: u32, k: u32, btc_to_i: &[u32], i_to_usd: &[u32]) -> u32 {
    use std::cmp::max;
    use rayon::prelude::*;

    btc_to_i.par_iter().copied().zip(i_to_usd.par_iter().copied())
        .map(|(bi, iu)| bi * iu)
        .reduce(|| k, |(best), bu| max(best, bu)) * m
}