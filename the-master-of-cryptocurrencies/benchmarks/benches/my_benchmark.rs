#[macro_use]
extern crate criterion;
extern crate the_master_of_cryptocurrencies;

use criterion::*;

use the_master_of_cryptocurrencies::*;

fn generate_input(n: u32) -> (u32, u32, Vec<u32>, Vec<u32>) {
    let a = (1..=n);
    let b = (1..=n).rev();
    (8, 7, a.collect(), b.collect())
}

fn criterion_benchmark(c: &mut Criterion) {
    for &i in [10u32, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000].iter() {
        let parameters = black_box(generate_input(i));
        let mut group = c.benchmark_group("the_master_of_cryptocurrencies");

        group.throughput(Throughput::Elements(i as u64));

        group.bench_with_input(BenchmarkId::new("solve_map_then_max", i), &parameters, |b, parameters| b.iter(|| {
            let (m, k, btc_to_i, i_to_usd) = parameters;

            solve_map_then_max(
                *m, *k, &btc_to_i, &i_to_usd,
            )
        }));

        group.bench_with_input(BenchmarkId::new("solve_fold", i), &parameters, |b, i| b.iter(|| {
            let (m, k, btc_to_i, i_to_usd) = i;

            solve_fold(
                *m, *k, &btc_to_i, &i_to_usd,
            )
        }));

        group.bench_with_input(BenchmarkId::new("solve_parallel", i), &parameters, |b, i| b.iter(|| {
            let (m, k, btc_to_i, i_to_usd) = i;

            solve_parallel(
                *m, *k, &btc_to_i, &i_to_usd,
            )
        }));

        group.bench_with_input(BenchmarkId::new("solve_parallel_reduce", i), &parameters, |b, i| b.iter(|| {
            let (m, k, btc_to_i, i_to_usd) = i;

            solve_parallel_reduce(
                *m, *k, &btc_to_i, &i_to_usd,
            )
        }));

        group.bench_with_input(BenchmarkId::new("solve_parallel_mapreduce", i), &parameters, |b, i| b.iter(|| {
            let (m, k, btc_to_i, i_to_usd) = i;

            solve_parallel_mapreduce(
                *m, *k, &btc_to_i, &i_to_usd,
            )
        }));

        group.finish();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);