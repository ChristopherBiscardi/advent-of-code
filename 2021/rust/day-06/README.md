# Advent of Code Day 05

## Part 1

### double for loops and lotsa alloc

```
365131
dhat: Total:     2,282,128 bytes in 677 blocks
dhat: At t-gmax: 680,536 bytes in 3 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

### VecDeque

```
❯ ./target/release/part1
365131
dhat: Total:     18,020 bytes in 12 blocks
dhat: At t-gmax: 8,856 bytes in 3 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

### parse nom with u8 instead of str

and don't use `str::parse<usize>()`

```
365131
dhat: Total:     2,716 bytes in 11 blocks
dhat: At t-gmax: 1,624 bytes in 2 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

## Part 2

```
did not finish
```

### VecDeque

```
day-06
❯ ./target/release/part2
1041836936
dhat: Total:     18,020 bytes in 12 blocks
dhat: At t-gmax: 8,856 bytes in 3 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```
