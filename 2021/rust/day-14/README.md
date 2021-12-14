# Advent of Code Day 11

## Part 1

dhat

```
3009
dhat: Total:     469,425 bytes in 19,545 blocks
dhat: At t-gmax: 152,133 bytes in 15 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

criterion

```
part1                   time:   [4.1651 ms 4.1826 ms 4.2019 ms]
```

### Comparison to using HashMap instead of BTreeMap

```
3009
dhat: Total:     468,905 bytes in 19,534 blocks
dhat: At t-gmax: 152,277 bytes in 6 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

```
part1                   time:   [1.7642 ms 1.7706 ms 1.7774 ms]
```

### By predicting the size of the String

```
3009
dhat: Total:     222,029 bytes in 43 blocks
dhat: At t-gmax: 138,822 bytes in 15 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

### By using .counts instead of .sorted().group_by()

```
3009
dhat: Total:     46,509 bytes in 35 blocks
dhat: At t-gmax: 31,543 bytes in 13 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

```
part1                   time:   [1.0155 ms 1.0214 ms 1.0273 ms]
```

### Using the part2 algorithm

```
3009
dhat: Total:     25,945 bytes in 114 blocks
dhat: At t-gmax: 6,773 bytes in 33 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

```
part1                   time:   [127.87 us 129.37 us 130.83 us]
```

## Part 2

dhat

```
2188189693529
dhat: Total:     27,818 bytes in 125 blocks
dhat: At t-gmax: 2,005 bytes in 10 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

criterion

```
part2                   time:   [482.08 us 485.98 us 490.06 us]
```

### Comparison to using HashMap instead of BTreeMap

```
2188189693529
dhat: Total:     42,538 bytes in 164 blocks
dhat: At t-gmax: 1,973 bytes in 5 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```
