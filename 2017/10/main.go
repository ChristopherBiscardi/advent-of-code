package main

import (
	"flag"
	"fmt"
	"log"
	"strconv"
	"strings"
)

func main() {
	flag.Parse()
	for _, v := range flag.Args() {
		stringSteps := strings.Split(v, ",")
		steps := []int{}
		for _, v := range stringSteps {
			digit, err := strconv.Atoi(v)
			if err != nil {
				log.Fatal("failed to convert digit", err)
			}
			steps = append(steps, digit)
		}

		result := Process(256, steps)
		fmt.Println(result)

	}
	//	fmt.Printf("total score: `%d`\n", result)
}

// ints.Do(func(x int) {
// 	fmt.Println(x)
// })

func Process(ringSize int, steps []int) int {
	ints := New(ringSize)

	curPos := 0

	for skip, length := range steps {
		move := length + skip
		ints.Reverse(length, curPos)

		// ring can deal with wraparound. % is for human
		// readability if we print debug
		curPos = (curPos + move) % ints.Len()
		fmt.Println(ints.Move(curPos).Value, move)
	}

	one := ints.Move(0).Value
	two := ints.Move(1).Value
	fmt.Println("0,1:", one, two)
	return one * two
}
