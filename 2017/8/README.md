# Day 8

[link](http://adventofcode.com/2017/day/8)

```
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
```

These instructions would be processed as follows:

* Because a starts at 0, it is not greater than 1, and so b is not modified.
* a is increased by 1 (to 1) because b is less than 5 (it is 0).
* c is decreased by -10 (to 10) because a is now greater than or equal to 1 (it
  is 1).
* c is increased by -20 (to -10) because c is equal to 10.

After this process, the largest value in any register is 1.

You might also encounter <= (less than or equal to) or != (not equal to).
However, the CPU doesn't have the bandwidth to tell you what all the registers
are named, and leaves that to you to determine.

# Solution

I got tired and rushed this problem without any tests. Was able to finish with
the only bug being a hardcoded string affecting the output.

# Running

The solution is written in golang and takes a list from stdin, printing the
answer on EOF (`C-d` on OSX).

```go
➜ go run main.go
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
max is 1
```
