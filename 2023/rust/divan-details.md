For some reason, when uncommenting the `part2_subject` module in `lib.rs`, then `part2_nom` _benchmark_ runs slower.

benchmarks with subject

```
day_01                 fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ part1               35.7 µs       │ 93.41 µs      │ 36.14 µs      │ 37.26 µs      │ 100     │ 100
├─ part2               134.3 µs      │ 163.5 µs      │ 134.7 µs      │ 137.1 µs      │ 100     │ 100
├─ part2_aho_corasick  26.7 ms       │ 37.09 ms      │ 27.08 ms      │ 27.31 ms      │ 100     │ 100
├─ part2_nom           467.4 µs      │ 568.4 µs      │ 472.3 µs      │ 480.6 µs      │ 100     │ 100
╰─ part2_subject       135.7 µs      │ 183.7 µs      │ 136.2 µs      │ 138.5 µs      │ 100     │ 100
```

benchmarks without subject

```
day_01                 fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ part1               33.58 µs      │ 99.66 µs      │ 33.99 µs      │ 35.05 µs      │ 100     │ 100
├─ part2               134.9 µs      │ 222.4 µs      │ 139.5 µs      │ 141.3 µs      │ 100     │ 100
├─ part2_aho_corasick  27.6 ms       │ 29.34 ms      │ 28.48 ms      │ 28.41 ms      │ 100     │ 100
╰─ part2_nom           361.2 µs      │ 455.5 µs      │ 375.9 µs      │ 378.3 µs      │ 100     │ 100

├─ part2_nom           467.4 µs      │ 568.4 µs      │ 472.3 µs      │ 480.6 µs      │ 100     │ 100
╰─ part2_nom           361.2 µs      │ 455.5 µs      │ 375.9 µs      │ 378.3 µs      │ 100     │ 100
```
