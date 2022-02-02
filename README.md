## Manual `memcpy` Benchmark

Benchmarks that compare copying data between two `Vec<u8>`s using
`std::slice::copy_from_slice` and a loop that copies one byte at a time.

### Usage
Run the benchmark with `cargo bench`.

### Results
Compiled with `rustc 1.60.0-nightly (498eeb72f 2022-01-31)`.
Run on Intel i7-7500U (2.70 GHz) with Ubuntu 21.04.
```
copy/16KiB_stdlib       time:   [2.0702 us 2.0888 us 2.1052 us]                               
Found 13 outliers among 100 measurements (13.00%)
  13 (13.00%) low mild
copy/16KiB_manual       time:   [1.9551 us 1.9761 us 1.9939 us]                               
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) low mild
copy/128KiB_stdlib      time:   [17.378 us 20.227 us 23.406 us]                               
Found 30 outliers among 100 measurements (30.00%)
  14 (14.00%) low mild
  2 (2.00%) high mild
  14 (14.00%) high severe
copy/128KiB_manual      time:   [13.238 us 13.361 us 13.463 us]                               
Found 11 outliers among 100 measurements (11.00%)
  11 (11.00%) low mild
copy/1MiB_stdlib        time:   [422.28 us 422.97 us 423.78 us]                             
Found 29 outliers among 100 measurements (29.00%)
  15 (15.00%) low severe
  5 (5.00%) low mild
  3 (3.00%) high mild
  6 (6.00%) high severe
copy/1MiB_manual        time:   [359.13 us 359.54 us 360.00 us]                             
Found 23 outliers among 100 measurements (23.00%)
  14 (14.00%) low severe
  5 (5.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe
copy/16MiB_stdlib       time:   [7.6757 ms 7.6886 ms 7.7045 ms]                              
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) high mild
  9 (9.00%) high severe
copy/16MiB_manual       time:   [6.3536 ms 6.3821 ms 6.4262 ms]                              
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe
```