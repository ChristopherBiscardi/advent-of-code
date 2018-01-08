# Day 12

[link](http://adventofcode.com/2017/day/12)

```
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
```

In this example, the following programs are in the group that contains program
ID 0:

* Program 0 by definition.
* Program 2, directly connected to program 0.
* Program 3 via program 2.
* Program 4 via program 2.
* Program 5 via programs 6, then 4, then 2.
* Program 6 via programs 4, then 2.

Therefore, a total of 6 programs are in this group; all but program 1, which has
a pipe that connects it to itself.

How many programs are in the group that contains program ID 0?

# Running

The solution is written in golang and takes a comma-separated list from stdin

```go
➜ go run main.go ne,ne,ne
3 steps away
```
