package main

import "testing"

func TestMove(t *testing.T) {
	moves := Move([]int{0, 3, 0, 1, -3})
	if moves != 5 {
		t.Errorf("For [0, 3, 0, 1, -3], Expected 5, got %d", moves)
	}

}
