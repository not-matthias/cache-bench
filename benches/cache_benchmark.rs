use atomic_cache_bench::{compare_exchange, compare_exchange_weak, load_store, unsafe_static, lazy_static_crate};
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

fn bench_unsafe_static(c: &mut Criterion) {
    c.bench_function("unsafe static 1", |b| b.iter(|| unsafe_static::cache_0()));
}


fn bench_lazy_static(c: &mut Criterion) {
    c.bench_function("lazy static 1", |b| b.iter(|| lazy_static_crate::cache_0()));
}

criterion_group!(benches, 
    bench_compare_exchange, bench_compare_exchange_weak,
     bench_load_store, bench_unsafe_static, bench_lazy_static);
criterion_main!(benches);
