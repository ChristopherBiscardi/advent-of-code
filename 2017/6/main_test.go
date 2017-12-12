package main

import (
	"testing"
)

var (
	testSlice = []int{0, 2, 7, 0}
)

func TestReallocate(t *testing.T) {
	slice := append([]int(nil), testSlice...)
	steps := Reallocate(slice)
	if steps != 5 {
		t.Errorf("For %v, Expected 5, got %d", slice, steps)
	}

}

func TestDistribute(t *testing.T) {
	slice := append([]int(nil), testSlice...)
	result := Distribute(slice)
	if result[0] != 2 ||
		result[1] != 4 ||
		result[2] != 1 ||
		result[3] != 2 {
		t.Errorf("result %v should be [2,4,1,2]")
	}
}

func TestDistributeSmaller(t *testing.T) {
	slice := []int{2, 1, 1, 1, 1}
	result := Distribute(slice)
	if result[0] != 0 ||
		result[1] != 2 ||
		result[2] != 2 ||
		result[3] != 1 ||
		result[4] != 1 {
		t.Errorf("result %v should be [0,2,2,1,1]", result)
	}

}

// test case where we have a max number less than then the number of banks
// and thus need to wrap around to index 0 to distribute
func TestDistributeWraparound(t *testing.T) {
	slice := []int{1, 1, 1, 4, 1}
	result := Distribute(slice)
	if result[0] != 2 ||
		result[1] != 2 ||
		result[2] != 2 ||
		result[3] != 0 ||
		result[4] != 2 {
		t.Errorf("result %v should be [2,2,2,0,2]", result)
	}

}

func TestMaxIndex(t *testing.T) {
	max, index := MaxIndex([]int{1, 3, 5, 3, 2})
	if max != 5 || index != 2 {
		t.Errorf("Expected 5, 2; Got %d, %d", max, index)
	}
}

func TestMaxIndexDupes(t *testing.T) {
	// gets the first max value index
	max, index := MaxIndex([]int{1, 3, 5, 3, 5})
	if max != 5 || index != 2 {
		t.Errorf("Expected 5, 2; Got %d, %d", max, index)
	}
}

func TextBanksToString(t *testing.T) {
	str := BanksToString([]int{1, 2, 3, 4})
	if str != "1,2,3,4" {
		t.Errorf("Expected `1,2,3,4`; Got %s", str)
	}
}
