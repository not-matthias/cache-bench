# cache-bench
Benchmarking  different cache implementations. Read more about this on my blog: https://not-matthias.github.io/posts/cache-bench/

## Results

```
cached-value/no cache   time:   [27.438 ns 27.823 ns 28.338 ns]
cached-value/once cell  time:   [216.02 ps 216.82 ps 217.75 ps]
cached-value/lazy static
                        time:   [320.65 ps 321.75 ps 323.15 ps]
cached-value/compare exchange
                        time:   [34.701 ns 34.811 ns 34.953 ns]
cached-value/load store time:   [234.55 ps 235.03 ps 235.62 ps]
cached-value/unsafe static
                        time:   [227.99 ps 228.74 ps 229.73 ps]
```

Microbench:
```
no cache (5.0s) ...                       27.115 ns/iter (0.999 R²)
once cell (5.0s) ...                       0.221 ns/iter (0.999 R²)
lazy static (5.0s) ...                     0.474 ns/iter (1.000 R²)
compare exchange (5.0s) ...               34.937 ns/iter (0.999 R²)
load store (5.0s) ...                      0.236 ns/iter (1.000 R²)
unsafe static (5.0s) ...                   0.226 ns/iter (1.000 R²)
```
