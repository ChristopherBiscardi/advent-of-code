# Day 2

[link](http://adventofcode.com/2017/day/2)

For example, given the following spreadsheet:

```
5 1 9 5
7 5 3
2 4 6 8
```

* The first row's largest and smallest values are 9 and 1, and their difference
  is 8.
* The second row's largest and smallest values are 7 and 3, and their difference
  is 4.
* The third row's difference is 6.
* In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.

# Running

The solution is written in golang and takes a grid from stdin, printing the
answer on EOF (`C-d` on OSX).

```go
➜ go run main.go
5 1 9 5
7 5 3
2 4 6 8
sum: 18
```
