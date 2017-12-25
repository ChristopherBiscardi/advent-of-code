package main

import (
	"testing"
)

func TestNew(t *testing.T) {
	slice := []int{0, 1, 2, 3, 4}
	ints := New(5)
	if !ints.EqualsSlice(slice) {
		t.Errorf("Expected top, got bottom\n%v\n%v", slice, ints.ToSlice())
	}
}

func TestReverse(t *testing.T) {
	slice := []int{4, 3, 2, 1, 0}
	ints := New(5).Reverse(5, 0)
	if !ints.EqualsSlice(slice) {
		t.Errorf("Expected top, got bottom\n%v\n%v", slice, ints.ToSlice())
	}
}
func TestReverseWrap(t *testing.T) {
	slice := []int{3, 2, 1, 0, 4}
	ints := New(5).Reverse(5, 2)
	if !ints.EqualsSlice(slice) {
		t.Errorf("Expected top, got bottom\n%v\n%v", slice, ints.ToSlice())
	}
}
