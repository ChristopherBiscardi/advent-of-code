# Day 1

[link](http://adventofcode.com/2017/day/1)

* 1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the
  second digit and the third digit (2) matches the fourth digit.
* 1111 produces 4 because each digit (all 1) matches the next.
* 1234 produces 0 because no digit matches the next.
* 91212129 produces 9 because the only digit that matches the next one is the
  last digit, 9.

# Running

The solution is written in golang and takes an arbitrary number of puzzles at a
time.

```go
➜ go run main.go 1122 1111 1234 91212129
3 : 1122
4 : 1111
0 : 1234
9 : 91212129
```

The above examples are also set up as tests

```shell
➜ go test
PASS
ok  	_/Users/biscarch/github/christopherbiscardi/advent-of-code/2017/1	0.005s
```

The verbose output turned out to be a bit much for the actual solution so I'd
probably change it to not output the input.
