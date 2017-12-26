package main

import "testing"

var (
	inputs = [][]string{
		[]string{"ne", "ne", "ne"},
		[]string{"ne", "ne", "sw", "sw"},
		[]string{"ne", "ne", "s", "s"},
		[]string{"se", "sw", "se", "sw", "sw"},
	}
	results = []int{3, 0, 2, 3}
)

func TestProcess(t *testing.T) {
	for i, v := range inputs {
		result := Process(v)
		if result != results[i] {
			t.Errorf("Expected %d, got %d", results[i], result)
		}
	}
}
