# Advent of Code Day 09

## Part 1

dhat

```
dhat: Total:     898,143 bytes in 1,021 blocks
dhat: At t-gmax: 196,963 bytes in 3 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

criterion

```
part1                   time:   [471.08 us 472.96 us 474.99 us]
                        change: [+565.47% +570.22% +575.04%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe

Benchmarking part2: Collecting 100 samples in estimated 5.28
```

### Using u8s

```
475
dhat: Total:     313,023 bytes in 1,021 blocks
dhat: At t-gmax: 56,815 bytes in 3 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

criterion

```
part1                   time:   [517.01 us 520.58 us 524.69 us]
```

### iter chain instead of collect and extend

```
475
dhat: Total:     252,623 bytes in 721 blocks
dhat: At t-gmax: 56,815 bytes in 3 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

```
part1                   time:   [464.71 us 466.46 us 468.42 us]
```

### iter chain in puzzle_input

```
475
dhat: Total:     190,403 bytes in 717 blocks
dhat: At t-gmax: 47,035 bytes in 41 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

```
part1                   time:   [497.04 us 499.94 us 503.25 us]
```

## Part 2

dhat

```
1092012
dhat: Total:     6,943,371 bytes in 10,221 blocks
dhat: At t-gmax: 3,445,459 bytes in 7,457 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

criterion

```
part2                   time:   [8.6051 ms 8.6406 ms 8.6793 ms]
                        change: [+288.48% +291.02% +293.51%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
```
