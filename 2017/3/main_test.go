package main

import "testing"

func TestSumLine(t *testing.T) {
	WithGoal(t, 12, 3)
	WithGoal(t, 23, 2)
	WithGoal(t, 1024, 31)
}

func TestAbsInt(t *testing.T) {
	if absInt(-5) != 5 {
		t.Errorf("Expected 5, got %d", absInt(-5))
	}
	if absInt(50) != 50 {
		t.Errorf("Expected 50, got %d", absInt(50))
	}
}
func WithGoal(t *testing.T, i int, result int) {
	distance := iterate(i)
	if distance != result {
		t.Errorf("For %d, Expected %d, got %d", i, result, distance)
	}
}
