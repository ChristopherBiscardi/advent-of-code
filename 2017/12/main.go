package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	input := [][]int{}
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		split := strings.Split(scanner.Text(), "<->")
		it := strings.Split(strings.Map(func(r rune) rune {
			if unicode.IsSpace(r) {
				return -1
			}
			return r
		}, split[1]), ",")

		ints := []int{}
		for _, v := range it {
			digit, err := strconv.Atoi(v)
			if err != nil {
				log.Fatal(err)
			}
			ints = append(ints, digit)
		}
		input = append(input, ints)
	}
	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		os.Exit(1)
	}

	fmt.Printf("%d members\n", Process(input))
}

// init the recursive algo
func Process(pipes [][]int) int {
	members := make(map[int]struct{})
	Iterate(pipes, members, 0)
	return len(members)
}

func Iterate(pipes [][]int, set map[int]struct{}, index int) {
	if _, ok := set[index]; ok {
		// number already exists, stop processing
		return
	} else {
		//number hasn't been registered yet
		set[index] = struct{}{}
	}
	for _, v := range pipes[index] {
		Iterate(pipes, set, v)
	}
}
