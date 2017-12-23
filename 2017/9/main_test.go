package main

import (
	"testing"
)

var (
	garbage = []string{
		`<>`,
		`<random characters>`,
		`<<<<>`,
		`<{!>}>`,
		`<!!>`,
		`<!!!>>`,
		`<{o"i!a,<{i<a>`,
	}
	groups = []string{
		`{}`,
		`{{{}}}`,
		`{{},{}}`,
		`{{{},{},{{}}}}`,
		`{<a>,<a>,<a>,<a>}`,
		`{{<ab>},{<ab>},{<ab>},{<ab>}}`,
		`{{<!!>},{<!!>},{<!!>},{<!!>}}`,
		`{{<a!>},{<a!>},{<a!>},{<ab>}}`,
	}
	groupsScores = []int{1, 6, 5, 16, 1, 9, 9, 3}
)

func TestEndToEnd(t *testing.T) {
	if false != false {
		t.Errorf("Expected root node to be `tknk`, got `%s`", "TODO")
	}
}

func TestParse(t *testing.T) {
	for i, v := range groups {
		total := Parse(v)
		if total != groupsScores[i] {
			t.Errorf("For index %d Expected %d, got %d; for input %s",
				i,
				groupsScores[i],
				total,
				v)
		}
	}
}
