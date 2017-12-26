# Day 11

[link](http://adventofcode.com/2017/day/11)

[here](http://keekerdc.com/2011/03/hexagon-grids-coordinate-systems-and-distance-calculations/)
is a good explanation of the approach I used.

```
ne,ne,ne is 3 steps away.
ne,ne,sw,sw is 0 steps away (back where you started).
ne,ne,s,s is 2 steps away (se,se).
se,sw,se,sw,sw is 3 steps away (s,s,sw).
```

# Running

The solution is written in golang and takes a comma-separated list from stdin

```go
➜ go run main.go ne,ne,ne
3 steps away
```
