use criterion::black_box;
use parallel_bit_counting::divide_and_conquer_count_ones::naive_parallel_count_ones_128;
use parallel_bit_counting::divide_and_conquer_count_ones::naive_parallel_count_ones_64;
use parallel_bit_counting::naive_count_ones::naive_count_bits_128;
use parallel_bit_counting::naive_count_ones::naive_count_bits_64;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use parallel_bit_counting::{calculate_mask, count_ones::count_ones};
use rand::Rng;

fn gen_random_input_u64(m: usize) -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::with_capacity(m);
    let mut rng = rand::thread_rng();
    for _ in 0..m {
        vec.push(rng.gen::<u64>())
    }
    vec
}
fn gen_random_input_u128(m: usize) -> Vec<u128> {
    let mut vec: Vec<u128> = Vec::with_capacity(m);
    for _ in 0..m {
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
            group.bench_with_input(
                BenchmarkId::new("Naïve Sequential Bit Counting, 64-bit", 1 << m),
                &input,
                |b, x| {
                    b.iter(|| {
                        black_box(naive_count_bits_64(&mut x.clone()));
                    })
                },
            );
        }
        {
            let input = gen_random_input_u64(1 << m);
            group.bench_with_input(
                BenchmarkId::new("Divide-And-Conquer Bit Counting, 64-bit", 1 << m),
                &input,
                |b, x| {
                    b.iter(|| {
                        black_box(naive_parallel_count_ones_64(x));
                    })
                },
            );
        }
        {
            let input = gen_random_input_u64(1 << m);
            group.bench_with_input(
                BenchmarkId::new("Parallel Bit Counting, 64-bit", 1 << m),
                &input,
                |b, x| {
                    b.iter(|| {
                        let masks = calculate_mask::get_mask(calculate_mask::get_basic_masks_u64());
                        count_ones(x, masks, parallel_bit_counting::count_ones::WordLength::U64);
                    })
                },
            );
        }
        {
            let input = gen_random_input_u128(1 << m);
            group.bench_with_input(
                BenchmarkId::new("Naïve Sequential Bit Counting, 128-bit", 1 << m),
                &input,
                |b, x| {
                    b.iter(|| {
                        black_box(naive_count_bits_128(&mut x.clone()));
                    })
                },
            );
        }
        {
            let input = gen_random_input_u128(1 << m);
            group.bench_with_input(
                BenchmarkId::new("Divide-And-Conquer Bit Counting, 128-bit", 1 << m),
                &input,
                |b, x| {
                    b.iter(|| {
                        black_box(naive_parallel_count_ones_128(x));
                    })
                },
            );
        }
        {
            let input = gen_random_input_u128(1 << m);
            group.bench_with_input(
                BenchmarkId::new("Parallel Bit Counting, 128-bit", 1 << m),
                &input,
                |b, x| {
                    b.iter(|| {
                        let masks =
                            calculate_mask::get_mask(calculate_mask::get_basic_masks_u128());
                        count_ones(
                            x,
                            masks,
                            parallel_bit_counting::count_ones::WordLength::U128,
                        );
                    })
                },
            );
        }
    }
}
criterion_group!(benches, benchmark);
criterion_main!(benches);
