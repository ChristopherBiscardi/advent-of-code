package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	var sum = 0
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		sum = sum + SumLine(scanner.Text())
	}
	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		os.Exit(1)
	}
	fmt.Printf("sum: %d", sum)
}

func SumLine(line string) int {
	var intSlice = []int{}
	for _, v := range strings.Fields(line) {
		digit, err := strconv.Atoi(v)
		if err != nil {
			fmt.Println(err)
			os.Exit(2)
		}
		intSlice = append(intSlice, digit)
	}
	min, max := MinAndMax(intSlice)
	return max - min
}

func MinAndMax(ints []int) (int, int) {
	var min = ints[0]
	var max = ints[0]
	for _, v := range ints {
		if v < min {
			min = v
		}
		if v > max {
			max = v
		}
	}
	return min, max
}
