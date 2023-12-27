# Parsing examples for AoC

This crate is meant to display different approaches. If you're optimizing microseconds off of your AoC problem, then you're probably going to want to write your own custom parser that _does not_ use any of these crates.

If you want to learn a parsing crate for easier Advent of Coding or real-world usage, then take your pick according to the API you like.

There are benchmarks here, but they aren't meant to be "end-all ultimate benchmarks". They're meant to be light overviews to compare on advent input.

```
day_02_parsing_bench         fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_nom_intro             29.62 µs      │ 98.41 µs      │ 29.83 µs      │ 30.95 µs      │ 100     │ 100
├─ big_nom_intro_bytes       22.08 µs      │ 30.74 µs      │ 22.16 µs      │ 22.43 µs      │ 100     │ 100
├─ big_nom_supreme_intro     48.62 µs      │ 70.7 µs       │ 50.31 µs      │ 50.97 µs      │ 100     │ 100
├─ big_pest_intro            113.6 µs      │ 195.2 µs      │ 114.1 µs      │ 116 µs        │ 100     │ 100
├─ big_regex_intro           1.006 ms      │ 1.594 ms      │ 1.016 ms      │ 1.03 ms       │ 100     │ 100
├─ big_winnow_intro          31.7 µs       │ 40.74 µs      │ 32.12 µs      │ 32.39 µs      │ 100     │ 100
├─ big_winnow_intro_bytes    28.12 µs      │ 36.91 µs      │ 29.12 µs      │ 29.23 µs      │ 100     │ 100
├─ small_nom_intro           812.1 ns      │ 921.5 ns      │ 838.1 ns      │ 841.6 ns      │ 100     │ 800
├─ small_nom_intro_bytes     629.8 ns      │ 713.3 ns      │ 635 ns        │ 641.8 ns      │ 100     │ 800
├─ small_nom_supreme_intro   1.405 µs      │ 1.905 µs      │ 1.468 µs      │ 1.483 µs      │ 100     │ 400
├─ small_pest_intro          4.082 µs      │ 9.583 µs      │ 4.291 µs      │ 4.354 µs      │ 100     │ 100
├─ small_regex_intro         418.9 µs      │ 455.9 µs      │ 423.7 µs      │ 425.5 µs      │ 100     │ 100
├─ small_winnow_intro        838.1 ns      │ 952.8 ns      │ 848.6 ns      │ 859.3 ns      │ 100     │ 800
╰─ small_winnow_intro_bytes  749.6 ns      │ 864.1 ns      │ 770.4 ns      │ 774.9 ns      │ 100     │ 800
```

## nom -> winnow upgrade benchmarks

As I ported from nom to winnow, I recorded benchmarks for each step of the way.

<details><summary>The state after the winnow 0.3 port for winnow_intro.</summary>

```
day_02_parsing_bench        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_nom_intro            30.49 µs      │ 74.41 µs      │ 30.74 µs      │ 31.56 µs      │ 100     │ 100
├─ big_nom_intro_bytes      22.37 µs      │ 33.79 µs      │ 22.49 µs      │ 23.06 µs      │ 100     │ 100
├─ big_nom_supreme_intro    44.95 µs      │ 52.33 µs      │ 45.24 µs      │ 45.79 µs      │ 100     │ 100
├─ big_winnow_intro         30.62 µs      │ 42.79 µs      │ 31.04 µs      │ 31.92 µs      │ 100     │ 100
├─ small_nom_intro          832.9 ns      │ 937.1 ns      │ 848.6 ns      │ 857.5 ns      │ 100     │ 800
├─ small_nom_intro_bytes    645.5 ns      │ 739.3 ns      │ 655.9 ns      │ 661 ns        │ 100     │ 800
├─ small_nom_supreme_intro  1.301 µs      │ 1.478 µs      │ 1.322 µs      │ 1.335 µs      │ 100     │ 400
╰─ small_winnow_intro       864.1 ns      │ 984 ns        │ 879.8 ns      │ 886.4 ns      │ 100     │ 800
```

</details>

<details><summary>The winnow 0.4 upgrade, continuing from above.</summary>

```
day_02_parsing_bench        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_nom_intro            31.16 µs      │ 72.58 µs      │ 33.29 µs      │ 33.24 µs      │ 100     │ 100
├─ big_nom_intro_bytes      22.45 µs      │ 31.91 µs      │ 23.2 µs       │ 23.58 µs      │ 100     │ 100
├─ big_nom_supreme_intro    41.29 µs      │ 49.58 µs      │ 41.87 µs      │ 42.78 µs      │ 100     │ 100
├─ big_winnow_intro         32.7 µs       │ 43.04 µs      │ 32.95 µs      │ 33.54 µs      │ 100     │ 100
├─ small_nom_intro          864.1 ns      │ 1.051 µs      │ 916.1 ns      │ 921.4 ns      │ 100     │ 400
├─ small_nom_intro_bytes    650.6 ns      │ 744.5 ns      │ 676.8 ns      │ 681.1 ns      │ 100     │ 800
├─ small_nom_supreme_intro  1.082 µs      │ 1.28 µs       │ 1.114 µs      │ 1.124 µs      │ 100     │ 400
╰─ small_winnow_intro       853.8 ns      │ 958 ns        │ 890.3 ns      │ 894.4 ns      │ 100     │ 800
```

</details>

<details><summary>winnow 0.5</summary>

```
day_02_parsing_bench        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ big_nom_intro            29.58 µs      │ 71.04 µs      │ 31.7 µs       │ 31.77 µs      │ 100     │ 100
├─ big_nom_intro_bytes      22.58 µs      │ 31.99 µs      │ 22.83 µs      │ 23.61 µs      │ 100     │ 100
├─ big_nom_supreme_intro    48.45 µs      │ 80.79 µs      │ 48.87 µs      │ 50.51 µs      │ 100     │ 100
├─ big_winnow_intro         31.62 µs      │ 44.16 µs      │ 32.08 µs      │ 32.7 µs       │ 100     │ 100
├─ small_nom_intro          806.9 ns      │ 1.999 µs      │ 838.1 ns      │ 857.9 ns      │ 100     │ 800
├─ small_nom_intro_bytes    661 ns        │ 859 ns        │ 687.1 ns      │ 695.7 ns      │ 100     │ 800
├─ small_nom_supreme_intro  1.385 µs      │ 4.207 µs      │ 1.426 µs      │ 1.458 µs      │ 100     │ 400
╰─ small_winnow_intro       827.8 ns      │ 890.3 ns      │ 859 ns        │ 855.3 ns      │ 100     │ 800
```

</details>
