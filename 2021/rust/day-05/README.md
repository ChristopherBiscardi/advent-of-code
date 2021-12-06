# Advent of Code Day 05

## Part 1

```
day-05
❯ ./target/release/part1
6113
dhat: Total:     2,323,027 bytes in 14,701 blocks
dhat: At t-gmax: 2,313,863 bytes in 14,692 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

## using i16 instead of i32 in Point

```
6113
dhat: Total:     1,727,266 bytes in 14,701 blocks
dhat: At t-gmax: 1,722,166 bytes in 14,692 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

### u8 in BTreeMap

```
➜ ./target/release/part1
6113
dhat: Total:     1,257,186 bytes in 14,701 blocks
dhat: At t-gmax: 1,252,086 bytes in 14,692 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

## Part 2

```
day-05
❯ ./target/release/part2
20373
dhat: Total:     3,773,491 bytes in 24,059 blocks
dhat: At t-gmax: 3,764,327 bytes in 24,050 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

## using i16 instead of i32 in Point

```
➜ ./target/release/part2
20373
dhat: Total:     2,803,410 bytes in 24,059 blocks
dhat: At t-gmax: 2,798,310 bytes in 24,050 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

### u8 in BTreeMap

```
➜ ./target/release/part2
20373
dhat: Total:     2,033,874 bytes in 24,059 blocks
dhat: At t-gmax: 2,028,774 bytes in 24,050 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```
