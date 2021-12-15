use core::fmt;
use std::fmt::Display;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use parallel_bit_counting::{
    calculate_mask,
    count_ones::{count_ones, WordLength},
};
use rand::Rng;

fn gen_random_input_u64(m: usize) -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::with_capacity(m);
    let mut rng = rand::thread_rng();
    for i in 0..m {
        vec.push(rng.gen::<u64>())
    }
    vec
}
fn gen_random_input_u128(m: usize) -> Vec<u128> {
    let mut vec: Vec<u128> = Vec::with_capacity(m);
    for i in 0..m {
        let mut rng = rand::thread_rng();
        vec.push(rng.gen::<u128>())
    }
    vec
}
fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parallel Bit-Counting");
    for m in 4..20 {
        group.throughput(Throughput::Elements(1 << m as u64));
        {
            let input = gen_random_input_u64(1 << m);
            group.bench_with_input(BenchmarkId::new("64-bit", 1 << m), &input, |b, x| {
                b.iter(|| {
                    let masks = calculate_mask::get_mask(calculate_mask::get_basic_masks_u64());
                    count_ones(x, masks);
                })
            });
        }
        {
            let input = gen_random_input_u128(1 << m);
            group.bench_with_input(BenchmarkId::new("128-bit", 1 << m), &input, |b, x| {
                b.iter(|| {
                    let masks = calculate_mask::get_mask(calculate_mask::get_basic_masks_u128());
                    count_ones(x, masks);
                })
            });
        }
    }
}
criterion_group!(benches, benchmark);
criterion_main!(benches);
