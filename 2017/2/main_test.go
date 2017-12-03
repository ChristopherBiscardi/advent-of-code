package main

import "testing"

const (
	one   = "5 1 9 5"
	two   = "7 5 3"
	three = "2 4 6 8"
)

func TestSumLine(t *testing.T) {
	WithLine(t, one, 8)
	WithLine(t, two, 4)
	WithLine(t, three, 6)
}

func TestMinAndMax(t *testing.T) {
	min, max := MinAndMax([]int{5, 1, 2, 3, 9})
	if min != 1 {
		t.Errorf("Expected %d, got %d", 1, min)
	}
	if max != 9 {
		t.Errorf("Expected %d, got %d", 1, max)
	}
}
func WithLine(t *testing.T, line string, result int) {
	sum := SumLine(line)
	if sum != result {
		t.Errorf("For '%s', Expected %d, got %d", line, result, sum)
	}
}
