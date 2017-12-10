# Day 3

[link](http://adventofcode.com/2017/day/3)

You come across an experimental new kind of memory stored on an infinite
two-dimensional grid.

Each square on the grid is allocated in a spiral pattern starting at a location
marked 1 and then counting up while spiraling outward. For example, the first
few squares are allocated like this:

```
17  16  15  14  13
18   5   4   3  12
19   6   1   2  11
20   7   8   9  10
21  22  23---> ...
```

While this is very space-efficient (no squares are skipped), requested data must
be carried back to square 1 (the location of the only access port for this
memory system) by programs that can only move up, down, left, or right. They
always take the shortest path: the Manhattan Distance between the location of
the data and square 1.

For example:

Data from square 1 is carried 0 steps, since it's at the access port. Data from
square 12 is carried 3 steps, such as: down, left, left. Data from square 23 is
carried only 2 steps: up twice. Data from square 1024 must be carried 31 steps.

# Manual Table Generation

I used this manually written table to check how movement should be evaluated.

```
xMax, xMin, yMax, yMin, curX, curY
0  0  0  0 :  0  0
1  0  0  0 :  1  0
1  0  1  0 :  1  1
1  0  1  0 :  0  1
1 -1  1  0 : -1  1
1 -1  1  0 : -1  0
1 -1  1 -1 : -1 -1
1 -1  1 -1 :  0 -1
1 -1  1 -1 :  1 -1
2 -1  1 -1 :  2 -1
```

# Running

```go
➜ go run main.go 312051 1024
430 : 312051
31 : 1024
```
