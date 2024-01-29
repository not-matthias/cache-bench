use atomic_cache_bench::{
    compare_exchange, lazy_static_crate, load_store, no_cache, once_cell_crate, unsafe_static,
};

use microbench::Options;

fn main() {
    let options = Options::default();
    microbench::bench(&options, "no cache", || no_cache::cached_value());
    microbench::bench(&options, "once cell", || once_cell_crate::cached_value());
    microbench::bench(&options, "lazy static", || {
        lazy_static_crate::cached_value()
    });
    microbench::bench(&options, "compare exchange", || {
        compare_exchange::cached_value()
    });
    microbench::bench(&options, "load store", || load_store::cached_value());
    microbench::bench(&options, "unsafe static", || unsafe_static::cached_value());
}
