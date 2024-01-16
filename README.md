# atomic-cache-bench
Benchmarking `comparing_exchange` against `load` and`store`

## Results

```
compare exchange 1      time:   [9.3459 ns 9.3816 ns 9.4186 ns]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) low mild
  3 (3.00%) high mild

compare exchange 2      time:   [6.8012 ns 6.8266 ns 6.8529 ns]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild

compare exchange weak 1 time:   [9.3877 ns 9.4253 ns 9.4628 ns]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe

compare exchange weak 2 time:   [6.9293 ns 6.9566 ns 6.9835 ns]
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) low severe
  4 (4.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe

load store 1            time:   [516.15 ps 518.73 ps 521.20 ps]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) low mild
  1 (1.00%) high mild
  3 (3.00%) high severe

load store 2            time:   [550.65 ps 596.71 ps 661.33 ps]
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe
```