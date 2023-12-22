26501365 = 2023(00) _ 131 + 65
step count = year _ line_length + step_interval

// this is the wrong sequence, but yields the
// quadratic insight anyway, and gets `a` correct
064: 3642
195: 33248
326: 92596
457: 181686
588: 300518
719: 449092

## first

❯ 33248 - 3642
29606

❯ 92596 - 33248
59348

❯ 181686 - 92596
89090

❯ 300518 - 181686
118832

❯ 449092 - 300518
148574

## second

❯ 59348 - 29606
29742

❯ 89090 - 59348
29742

❯ 118832 - 89090
29742

advent-of-code/2023/rust
❯ 148574 - 118832
29742

third: all zeros! that means quadratic

---

half the second

❯ 29742 / 2
14871 is a?

// the correct sequence is this, if you skip 000
000: 1
065: 3776
196: 33652
327: 93270
458: 182630
589: 301732

---

202300

26501365 STEPS is what number in the sequence?
26501365 - 65 / 131
