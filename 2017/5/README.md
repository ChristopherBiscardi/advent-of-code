# Day 5

[link](http://adventofcode.com/2017/day/5)

```
0
3
0
1
-3
```

Positive jumps ("forward") move downward; negative jumps move upward. For
legibility in this example, these offset values will be written all on one line,
with the current instruction marked in parentheses. The following steps would be
taken before an exit is found:

* `(0) 3 0 1 -3` - before we have taken any steps.
* `(1) 3 0 1 -3` - jump with offset 0 (that is, don't jump at all). Fortunately,
  the instruction is then incremented to 1.
* `2 (3) 0 1 -3` - step forward because of the instruction we just modified. The
  first instruction is incremented again, now to 2.
* `2 4 0 1 (-3)` - jump all the way to the end; leave a 4 behind.
* `2 (4) 0 1 -2` - go back to where we just were; increment -3 to -2.
* `2 5 0 1 -2` - jump 4 steps forward, escaping the maze.

# Running

The input comes as a column of numbers, so we use tr to replace newlines with
spaces.

```go
➜ go run main.go 0 3 0 1 -3
5 moves
```

```go
➜ go run main.go $(pbpaste | tr '\n' ' ')
315613 moves
```
