package main

import (
	"flag"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	banks := StdinToSlice()
	fmt.Printf("%d moves\n", Reallocate(banks))
}

// create a slice from multiple arguments
func StdinToSlice() []int {
	slice := []int{}
	flag.Parse()
	for _, v := range flag.Args() {
		digit, err := strconv.Atoi(v)
		if err != nil {
			fmt.Println(err)
			os.Exit(2)
		}
		slice = append(slice, digit)
	}
	return slice
}

func Reallocate(banks []int) int {
	banksSeen := make(map[string]struct{})
	banksSeen[BanksToString(banks)] = struct{}{}

	steps := 0

	for true {
		steps = steps + 1
		bankStr := BanksToString(Distribute(banks))
		_, exists := banksSeen[bankStr]
		if exists {
			break
		} else {
			banksSeen[bankStr] = struct{}{}
		}
	}

	return steps
}

func Distribute(banks []int) []int {
	value, index := MaxIndex(banks)
	banks[index] = 0

	common := value / (len(banks) - 1)
	remainder := value % (len(banks) - 1)

	if common != 0 {
		banks[index] = remainder
	}
	if common == 0 {
		// value doesn't reach back to original index
		//		log.Fatal("not enough to share")
		curVal := value
		curIndex := index + 1
		for curVal > 0 {
			if curIndex == len(banks) {
				curIndex = 0
			}
			curVal = curVal - 1
			banks[curIndex] = banks[curIndex] + 1
			if curIndex != len(banks) {
				curIndex = curIndex + 1
			}

		}
	} else {
		for i, _ := range banks {
			if i != index {
				banks[i] = banks[i] + common
			}
		}
	}
	return banks
}

func MaxIndex(ints []int) (int, int) {
	maxInt := ints[0]
	index := 0
	for i, e := range ints {
		if e > maxInt {
			maxInt = e
			index = i
		}
	}
	return maxInt, index
}

// turn an []int into a keyable string
func BanksToString(banks []int) string {
	strBank := []string{}
	for _, v := range banks {
		strBank = append(strBank, strconv.Itoa(v))
	}
	return strings.Join(strBank, ",")
}
