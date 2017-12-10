# Day 3

[link](http://adventofcode.com/2017/day/3)

For example:

* aa bb cc dd ee is valid.
* aa bb cc dd aa is not valid - the word aa appears more than once.
* aa bb cc dd aaa is valid - aa and aaa count as different words.

# Running

The solution is written in golang and takes a grid from stdin, printing the
answer on EOF (`C-d` on OSX).

```go
➜ go run main.go
337 valid passphrases
```
