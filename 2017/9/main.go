package main

import (
	"flag"
	"fmt"
	"io/ioutil"
	"log"
)

func main() {
	flag.Parse()
	input, err := ioutil.ReadFile("./input.txt")
	if err != nil {
		log.Fatal("couldn't read file", err)
	}

	total := Parse(string(input))

	fmt.Printf("total score: `%d`\n", total)
}

func Parse(str string) int {
	level := 0
	total := 0

	shouldSkip := false
	inGarbage := false
	for _, v := range str {
		if shouldSkip {
			shouldSkip = false
		} else {
			switch {
			case '!' == v:
				// skip the next value
				shouldSkip = true
			case '{' == v && !inGarbage:
				// safe to add a child
				// fmt.Printf("adding child for %s\n", str)
				level += 1
				total += level
			case '}' == v && !inGarbage:
				//close existing group by no longer
				// working on it
				if level != 0 {
					level--
				}
			case '<' == v && !inGarbage:
				inGarbage = true
			case '>' == v && inGarbage:
				inGarbage = false
			default:
			}
		}
	}
	return total
}
