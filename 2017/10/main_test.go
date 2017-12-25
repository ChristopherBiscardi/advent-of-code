package main

import "testing"

func TestIncrementalProcess(t *testing.T) {
	// Test a full process incrementally
	ints := New(5)
	steps := []int{3, 4, 1, 5}
	offsets := []int{0, 3, 3, 6}
	results := [][]int{
		[]int{2, 1, 0, 3, 4},
		[]int{4, 3, 0, 1, 2},
		[]int{4, 3, 0, 1, 2},
		[]int{3, 4, 2, 1, 0},
	}
	for i, v := range steps {
		ints.Reverse(v, offsets[i])
		if !ints.EqualsSlice(results[i]) {
			t.Errorf("For step %d, Expected top, got bottom\n     %v\n%v", i, results[i], ints)
		}

	}
}
func TestProcess(t *testing.T) {
	// Test a full Process
	result := Process(5, []int{3, 4, 1, 5})
	if result != 12 {
		t.Errorf("Expected result to be 12, got %d", result)
	}
}
