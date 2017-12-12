package main

import (
	"flag"
	"fmt"
	"os"
	"strconv"
)

func main() {
	steps := []int{}
	flag.Parse()
	for _, v := range flag.Args() {
		digit, err := strconv.Atoi(v)
		if err != nil {
			fmt.Println(err)
			os.Exit(2)
		}
		steps = append(steps, digit)
	}
	fmt.Printf("%d moves", Move(steps))

}

func Move(ints []int) int {
	index := 0
	moves := 0
	for index < len(ints) && index >= 0 {
		moves = moves + 1
		ints[index] = ints[index] + 1
		index = index + ints[index] - 1

	}
	return moves
}
