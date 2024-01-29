#![no_std]

use core::{
    hint::black_box,
    sync::atomic::{AtomicU64, Ordering},
};

#[cold]
fn calc_value() -> u64 {
    black_box(x86::cpuid::cpuid!(0).eax as u64)
}

pub mod no_cache {
    use super::*;

    pub fn cached_value() -> u64 {
        calc_value()
    }
}

pub mod once_cell_crate {
    use once_cell::sync::OnceCell;

    use super::*;

    pub fn cached_value() -> u64 {
        static CACHED_VALUE: OnceCell<u64> = OnceCell::new();
        *CACHED_VALUE.get_or_init(calc_value)
    }
}

pub mod lazy_static_crate {
    use lazy_static::lazy_static;

    use super::*;

    pub fn cached_value() -> u64 {
        lazy_static! {
            static ref CACHED_VALUE: u64 = calc_value();
        }

        *CACHED_VALUE
    }
}

pub mod compare_exchange {
    use super::*;

    pub fn cached_value() -> u64 {
        static CACHED_VALUE: AtomicU64 = AtomicU64::new(u64::MAX);

        let _ = CACHED_VALUE.compare_exchange(
            u64::MAX,
            calc_value(),
            Ordering::Acquire,
            Ordering::Relaxed,
        );

        CACHED_VALUE.load(Ordering::Acquire)
    }
}

pub mod load_store {
    use super::*;

    pub fn cached_value() -> u64 {
        static CACHED_VALUE: AtomicU64 = AtomicU64::new(u64::MAX);

        let value = CACHED_VALUE.load(Ordering::Acquire);
        if value != u64::MAX {
            return value;
        }

        let value = calc_value();
        CACHED_VALUE.store(value, Ordering::Release);
        value
    }
}

pub mod unsafe_static {
    use super::*;

    pub fn cached_value() -> u64 {
        static mut CACHED_VALUE: u64 = u64::MAX;

        let value = unsafe { CACHED_VALUE };
        if value != u64::MAX {
            return value;
        }

        let value = calc_value();
        unsafe { CACHED_VALUE = value };
        value
    }
}
