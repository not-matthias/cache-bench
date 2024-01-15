use atomic_cache_bench::cache_2;
use atomic_cache_bench::{cache_1, cache_3, cache_4};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_compare_exchange(c: &mut Criterion) {
    c.bench_function("compare exchange 1", |b| b.iter(|| cache_1()));
    c.bench_function("compare exchange 2", |b| b.iter(|| cache_2()));
}

fn bench_load_store(c: &mut Criterion) {
    c.bench_function("load store 1", |b| b.iter(|| cache_3()));
    c.bench_function("load store 2", |b| b.iter(|| cache_4()));
}

criterion_group!(benches, bench_compare_exchange, bench_load_store);
criterion_main!(benches);
