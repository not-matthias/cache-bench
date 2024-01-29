use atomic_cache_bench::{
    compare_exchange, lazy_static_crate, load_store, no_cache, once_cell_crate, unsafe_static,
};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_grouped(c: &mut Criterion) {
    let mut g = c.benchmark_group("cached-value");

    g.bench_function("no cache", |b| b.iter(|| no_cache::cached_value()));
    g.bench_function("once cell", |b| b.iter(|| once_cell_crate::cached_value()));
    g.bench_function("lazy static", |b| {
        b.iter(|| lazy_static_crate::cached_value())
    });
    g.bench_function("compare exchange", |b| {
        b.iter(|| compare_exchange::cached_value())
    });
    g.bench_function("load store", |b| b.iter(|| load_store::cached_value()));
    g.bench_function("unsafe static", |b| {
        b.iter(|| unsafe_static::cached_value())
    });

    g.finish();
}

criterion_group!(benches, bench_grouped);
criterion_main!(benches);
