# Day 10

[link](http://adventofcode.com/2017/day/10)

Suppose we instead only had a circular list containing five elements, 0, 1, 2,
3, 4, and were given input lengths of 3, 4, 1, 5.

* [0] 1 2 3 4
* (length 3) capture (0 1 2)
* 2 1 0 [3] 4
* (length 4) capture (3 4 2 1)
* 4 3 0 [1] 2
* (length 1) capture (1)
* 4 [3] 0 1 2
* (length 5) capture (3 0 1 2 4)
* 3 4 2 1 [0]

# Running

The solution is written in golang and takes a list from stdin, printing the
answer on EOF (`C-d` on OSX).

```go
➜ go run main.go number_ring.go 165,1,255,31,87,52,24,113,0,91,148,254,158,2,73,153
4114
```
