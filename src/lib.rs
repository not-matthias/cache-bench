use std::sync::atomic::{AtomicU64, Ordering};

fn calc_value() -> u64 {
    42
}


pub mod compare_exchange {
    use super::*;

    pub fn cache_0() -> u64 {
        static CACHED_VALUE: AtomicU64 = AtomicU64::new(u64::MAX);

        let _ =
            CACHED_VALUE.compare_exchange(u64::MAX, calc_value(), Ordering::Acquire, Ordering::Relaxed);

        CACHED_VALUE.load(Ordering::Acquire)
    }

    pub fn cache_1() -> u64 {
        static CACHED_VALUE: AtomicU64 = AtomicU64::new(u64::MAX);

        let calc_value = calc_value();
        match CACHED_VALUE.compare_exchange(u64::MAX, calc_value, Ordering::Acquire, Ordering::Relaxed)
        {
            Ok(_) => {
                // Write was successful
                calc_value
            }
            Err(value) => value,
        }
    }
}

pub mod compare_exchange_weak {
    use super::*;


    pub fn cache_0() -> u64 {
        static CACHED_VALUE: AtomicU64 = AtomicU64::new(u64::MAX);

        let _ =
            CACHED_VALUE.compare_exchange_weak(u64::MAX, calc_value(), Ordering::Relaxed, Ordering::Relaxed);

        CACHED_VALUE.load(Ordering::Relaxed)
    }

    pub fn cache_1() -> u64 {
        static CACHED_VALUE: AtomicU64 = AtomicU64::new(u64::MAX);

        let calc_value = calc_value();
        match CACHED_VALUE.compare_exchange_weak(u64::MAX, calc_value, Ordering::Relaxed, Ordering::Relaxed)
        {
            Ok(_) => {
                // Write was successful
                calc_value
            }
            Err(value) => value,
        }
    }
}

pub mod load_store {
    use super::*;

    pub fn cache_0() -> u64 {
        static CACHED_VALUE: AtomicU64 = AtomicU64::new(u64::MAX);

        let value = CACHED_VALUE.load(Ordering::Acquire);
        if value != u64::MAX {
            return value;
        }

        let value = calc_value();
        CACHED_VALUE.store(value, Ordering::Release);
        value
    }

    pub fn cache_1() -> u64 {
        static CACHED_VALUE: AtomicU64 = AtomicU64::new(u64::MAX);

        let value = CACHED_VALUE.load(Ordering::Relaxed);
        if value != u64::MAX {
            return value;
        }

        let value = calc_value();
        CACHED_VALUE.store(value, Ordering::Relaxed);
        value
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values() {
        assert_eq!(compare_exchange::cache_0(), calc_value());
        assert_eq!(compare_exchange::cache_1(), calc_value());
        assert_eq!(compare_exchange_weak::cache_0(), calc_value());
        assert_eq!(compare_exchange_weak::cache_1(), calc_value());
        assert_eq!(load_store::cache_0(), calc_value());
        assert_eq!(load_store::cache_1(), calc_value());
    }
}
