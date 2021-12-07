# Advent of Code Day 07

## Part 1

### original

```
❯ ./target/release/part1
352707
dhat: Total:     14,883 bytes in 98 blocks
dhat: At t-gmax: 11,807 bytes in 88 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

criterion

```
part1                   time:   [7.1576 ms 7.1965 ms 7.2362 ms]
                        change: [+1.0644% +1.7251% +2.3822%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

## Part 2

### original

```
day-07
❯ ./target/release/part2
95519693
dhat: Total:     14,883 bytes in 98 blocks
dhat: At t-gmax: 11,807 bytes in 88 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

criterion

```
part2                   time:   [8.1613 ms 8.2117 ms 8.2614 ms]
                        change: [+0.3769% +1.0881% +1.8888%] (p = 0.00 < 0.05)
                        Change within noise threshold.
```

with arithmetic sum:

```
part2                   time:   [7.0574 ms 7.0844 ms 7.1127 ms]
                        change: [-14.350% -13.728% -13.080%] (p = 0.00 < 0.05)
                        Performance has improved.
```
