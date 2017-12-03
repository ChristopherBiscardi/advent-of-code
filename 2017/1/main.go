package main

import (
	"flag"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	flag.Parse()
	for _, v := range flag.Args() {
		fmt.Printf("%d : %s\n", Matcher(v), v)
	}
}

func Matcher(s string) int {
	stringsSlice := strings.Split(s, "")
	var matches = []int{}
	for i, v := range stringsSlice {
		// If we're on the last item, check against the first item
		if i == len(stringsSlice)-1 {
			if v == stringsSlice[0] {
				digit, err := strconv.Atoi(stringsSlice[i])
				if err != nil {
					fmt.Println(err)
					os.Exit(2)
				}
				matches = append(matches, digit)
			}
			break
		}
		if v == stringsSlice[i+1] {
			digit, err := strconv.Atoi(stringsSlice[i])
			if err != nil {
				fmt.Println(err)
				os.Exit(2)
			}
			matches = append(matches, digit)
		}
	}
	sum := 0
	for _, v := range matches {
		sum = sum + v
	}
	return sum
}
