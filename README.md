# atomic-cache-bench
Benchmarking  different cache implementations.

## Results

```
cached-value/no cache           time:   [28.142 ns 28.465 ns 28.861 ns]
cached-value/once cell          time:   [1.0065 ns 1.0551 ns 1.1192 ns]
cached-value/lazy static        time:   [1.1944 ns 1.2539 ns 1.3328 ns]
cached-value/compare exchange   time:   [33.951 ns 34.068 ns 34.193 ns]
cached-value/load store         time:   [869.90 ps 873.37 ps 877.06 ps]
cached-value/unsafe static      time:   [1.1938 ns 1.2783 ns 1.3793 ns]
```