package main

import (
	"strings"
	"testing"
)

const (
	testInput = `pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)`
)

func TestEndToEnd(t *testing.T) {

	isParentSet = make(map[string]struct{})
	isChildSet = make(map[string]struct{})

	for _, v := range strings.Split(testInput, "\n") {
		InsertNodes(&isParentSet, &isChildSet, v)
	}
	headNode := GetHeadNode(isParentSet, isChildSet)
	if headNode != "tknk" {
		t.Errorf("Expected root node to be `tknk`, got `%s`", headNode)
	}
}

func TestInsertNodes(t *testing.T) {
	isParentSet = make(map[string]struct{})
	isChildSet = make(map[string]struct{})

	for _, v := range strings.Split(testInput, "\n") {
		InsertNodes(&isParentSet, &isChildSet, v)
	}

	for _, v := range []string{"tknk", "fwft", "padx", "ugml"} {
		_, exists := isParentSet[v]
		if !exists {
			keys := make([]string, 0, len(isParentSet))
			for k := range isParentSet {
				keys = append(keys, k)
			}
			t.Errorf("Expected `%s` to be a value in parent set `%q`", v, keys)
		}

	}

	for _, v := range []string{"pbga", "xhth", "ebii", "havc", "ktlj", "ktlj", "cntj", "xhth", "qoyq", "pbga", "havc", "qoyq", "ugml", "padx", "fwft", "jptl", "gyxo", "ebii", "jptl", "gyxo", "cntj"} {
		_, exists := isChildSet[v]
		if !exists {
			t.Errorf("Expected `%s` to be a value in child set: `%q`", v, isChildSet)
		}

	}
}

func TestSplitLine(t *testing.T) {
	node := SplitLine("fwft (72)")
	if node.IsParent != false {
		t.Errorf("For `fwft (72)`, Expected to not be parent, got %v", node)
	}

	if node.Name != "fwft" {
		t.Errorf("For `fwft (72)`, Expected `fwft`, got %v", node)
	}
}

func TestSplitLineDoubleMatch(t *testing.T) {
	input := "fwft (72) -> ktlj, cntj, xhth"
	secondMatch := []string{"ktlj", "cntj", "xhth"}
	firstMatch := "fwft"

	node := SplitLine(input)

	for i, str := range secondMatch {
		if node.Children[i] != str {
			t.Errorf("For `%s`, Expected `%s` for second match, got %v", input, str, node.Children[i])
		}

	}

	if node.Name != firstMatch {
		t.Errorf("For `%s`, Expected `%s`, got %v", input, firstMatch, node.Name)
	}
}
