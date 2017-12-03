package main

import "testing"

func TestMatcher(t *testing.T) {
	WithMatch(t, "1122", 3)
	WithMatch(t, "1111", 4)
	WithMatch(t, "1234", 0)
	WithMatch(t, "91212129", 9)
}

func WithMatch(t *testing.T, match string, result int) {
	var sum = Matcher(match)
	if sum != result {
		t.Errorf("For '%s', Expected %d, got %d", match, result, sum)
	}
}
