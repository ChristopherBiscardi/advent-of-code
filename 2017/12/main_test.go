package main

import "testing"

var (
	input = [][]int{
		[]int{2},
		[]int{1},
		[]int{0, 3, 4},
		[]int{2, 4},
		[]int{2, 3, 6},
		[]int{6},
		[]int{4, 5},
	}
	result = 6
)

func TestProcess(t *testing.T) {
	output := Process(input)
	if output != result {
		t.Errorf("Expected %d, got %d", result, output)
	}
}
