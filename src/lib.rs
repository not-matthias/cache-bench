use std::sync::atomic::{AtomicU64, Ordering};

fn calc_value() -> u64 {
    42
}

pub fn cache_1() -> u64 {
    static CACHED_VALUE: AtomicU64 = AtomicU64::new(u64::MAX);

    let _ =
        CACHED_VALUE.compare_exchange(u64::MAX, calc_value(), Ordering::Relaxed, Ordering::Relaxed);

    CACHED_VALUE.load(Ordering::Relaxed)
}

pub fn cache_2() -> u64 {
    static CACHED_VALUE: AtomicU64 = AtomicU64::new(u64::MAX);

    let calc_value = calc_value();
    match CACHED_VALUE.compare_exchange(u64::MAX, calc_value, Ordering::Relaxed, Ordering::Relaxed)
    {
        Ok(_) => {
            // Write was successful
            calc_value
        }
        Err(value) => value,
    }
}

pub fn cache_3() -> u64 {
    static CACHED_VALUE: AtomicU64 = AtomicU64::new(u64::MAX);

    let value = CACHED_VALUE.load(Ordering::Relaxed);
    if value != u64::MAX {
        return value;
    }

    let value = calc_value();
    CACHED_VALUE.store(value, Ordering::Relaxed);
    value
}

pub fn cache_4() -> u64 {
    static CACHED_VALUE: AtomicU64 = AtomicU64::new(u64::MAX);

    let value = CACHED_VALUE.load(Ordering::SeqCst);
    if value != u64::MAX {
        return value;
    }

    let value = calc_value();
    CACHED_VALUE.store(value, Ordering::SeqCst);
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_values() {
        assert_eq!(cache_1(), calc_value());
        assert_eq!(cache_2(), calc_value());
        assert_eq!(cache_3(), calc_value());
        assert_eq!(cache_4(), calc_value());
    }
}
