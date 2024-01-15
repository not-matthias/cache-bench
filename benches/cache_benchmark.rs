use atomic_cache_bench::{compare_exchange, compare_exchange_weak, load_store};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_compare_exchange(c: &mut Criterion) {
    c.bench_function("compare exchange 1", |b| b.iter(|| compare_exchange::cache_0()));
    c.bench_function("compare exchange 2", |b| b.iter(|| compare_exchange::cache_1()));
}

fn bench_compare_exchange_weak(c: &mut Criterion) {
    c.bench_function("compare exchange weak 1", |b| b.iter(|| compare_exchange_weak::cache_0()));
    c.bench_function("compare exchange weak 2", |b| b.iter(|| compare_exchange_weak::cache_1()));
}

fn bench_load_store(c: &mut Criterion) {
    c.bench_function("load store 1", |b| b.iter(|| load_store::cache_0()));
    c.bench_function("load store 2", |b| b.iter(|| load_store::cache_1()));
}

criterion_group!(benches, bench_compare_exchange, bench_compare_exchange_weak, bench_load_store);
criterion_main!(benches);
