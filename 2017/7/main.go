package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strings"
)

var (
	loneOrHasChildrenRegex = regexp.MustCompile("^([a-z]*) \\([0-9]*\\)(?:$| -> (.*))")
	isParentSet            = make(map[string]struct{})
	isChildSet             = make(map[string]struct{})
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		InsertNodes(&isParentSet, &isChildSet, scanner.Text())
	}
	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		os.Exit(1)
	}

	fmt.Printf("head node: `%s`\n", GetHeadNode(isParentSet, isChildSet))
}

func GetHeadNode(pSet map[string]struct{}, cSet map[string]struct{}) string {
	for k := range isParentSet {
		if _, exists := isChildSet[k]; !exists {
			// early return if we find a value
			return k
		}
	}
	log.Fatal("All parents are Children")
	return ""
}

type Node struct {
	Name     string
	IsParent bool
	Children []string
}

func InsertNodes(pSet *map[string]struct{}, cSet *map[string]struct{}, str string) {
	node := SplitLine(str)
	if node.IsParent {
		(*pSet)[node.Name] = struct{}{}
	}
	if len(node.Children) > 0 {
		for _, child := range node.Children {
			(*cSet)[child] = struct{}{}
		}
	}
	return
}

func SplitLine(str string) Node {
	matches := loneOrHasChildrenRegex.FindAllStringSubmatch(str, -1)[0]
	if matches[2] == "" {
		return Node{matches[1], false, []string{}}
	}
	childMatches := strings.FieldsFunc(matches[2], func(r rune) bool {
		switch r {
		case ',', ' ':
			return true
		}
		return false
	})
	return Node{matches[1], true, childMatches}
}
