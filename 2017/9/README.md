# Day 9

[link](http://adventofcode.com/2017/day/9)

Here are some self-contained pieces of garbage:

* `<>`, empty garbage.
* `<random characters>`, garbage containing random characters.
* `<<<<>`, because the extra < are ignored.
* `<{!>}>`, because the first > is canceled.
* `<!!>`, because the second ! is canceled, allowing the > to terminate the
  garbage.
* `<!!!>>`, because the second ! and the first > are canceled.
* `<{o"i!a,<{i<a>`, which ends at the first >.

Here are some examples of whole streams and the number of groups they contain:

* `{}`, 1 group.
* `{{{}}}`, 3 groups.
* `{{},{}}`, also 3 groups.
* `{{{},{},{{}}}}`, 6 groups.
* `{<{},{},{{}}>}`, 1 group (which itself contains garbage).
* `{<a>,<a>,<a>,<a>}`, 1 group.
* `{{<a>},{<a>},{<a>},{<a>}}`, 5 groups.
* `{{<!>},{<!>},{<!>},{<a>}}`, 2 groups (since all but the last > are canceled).

Your goal is to find the total score for all groups in your input. Each group is
assigned a score which is one more than the score of the group that immediately
contains it. (The outermost group gets a score of 1.)

* `{}`, score of `1`.
* `{{{}}}`, score of `1 + 2 + 3 = 6`.
* `{{},{}}`, score of `1 + 2 + 2 = 5`.
* `{{{},{},{{}}}}`, score of `1 + 2 + 3 + 3 + 3 + 4 = 16`.
* `{<a>,<a>,<a>,<a>}`, score of `1`.
* `{{<ab>},{<ab>},{<ab>},{<ab>}}`, score of `1 + 2 + 2 + 2 + 2 = 9`.
* `{{<!!>},{<!!>},{<!!>},{<!!>}}`, score of `1 + 2 + 2 + 2 + 2 = 9`.
* `{{<a!>},{<a!>},{<a!>},{<ab>}}`, score of `1 + 2 = 3`.

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
