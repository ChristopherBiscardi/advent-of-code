package main

import (
	"testing"
)

var ()

func TestProcess(t *testing.T) {
	m := map[int]Segment{
		0: NewSegment(3),
		1: NewSegment(2),
		4: NewSegment(4),
		6: NewSegment(4),
	}
	result := 4
	score := Process(m, 6)
	if score != result {
		t.Errorf("Expected %d, got %d", result, score)
	}
}

func TestSimulateStep(t *testing.T) {
	errout := false
	segmentValues := []int{}
	steps := []int{1, 2, 1, 0}
	segment := NewSegment(3)
	for _, v := range steps {
		segment.SimulateStep()
		segmentValues = append(segmentValues, segment.Value)
		if segment.Value != v {
			errout = true
		}
	}
	if errout {
		t.Errorf("Expected %v, got %v", steps, segmentValues)
	}
}

func TestSimulateStepMap(t *testing.T) {
	m := map[int]Segment{
		0: NewSegment(3),
		1: NewSegment(2),
		4: NewSegment(4),
		6: NewSegment(4),
	}
	SimulateStepMap(m)
	for _, k := range []int{0, 1, 4, 6} {
		if m[k].Value != 1 {
			t.Errorf("Expected Simulated Map index %d to all be 1, got %d", k, m[k].Value)
		}
	}
}
