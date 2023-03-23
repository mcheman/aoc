use criterion::{criterion_group, criterion_main, Criterion};

use aoc::one;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("One", |b| b.iter(|| one()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);