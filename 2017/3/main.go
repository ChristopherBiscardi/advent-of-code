package main

import (
	"flag"
	"fmt"
	"os"
	"strconv"
)

func main() {
	flag.Parse()
	for _, v := range flag.Args() {
		digit, err := strconv.Atoi(v)
		if err != nil {
			fmt.Println(err)
			os.Exit(2)
		}
		fmt.Printf("%d : %s\n", iterate(digit), v)
	}
}
func iterate(to int) int {
	xMax, xMin, yMax, yMin := 0, 0, 0, 0
	curX, curY := 0, 0
	for i := 0; i < to-1; i++ {
		//		fmt.Println(curX, curY)
		if shouldMoveRight(xMax, xMin, yMax, yMin) {
			curX = curX + 1
			if curX > xMax {
				xMax = curX
			}
		} else if shouldMoveUp(xMax, xMin, yMax, yMin) {
			curY = curY + 1
			if curY > yMax {
				yMax = curY
			}
		} else if shouldMoveDown(xMax, xMin, yMax, yMin) {
			curY = curY - 1
			if -curY > yMin {
				yMin = -curY
			}
		} else if shouldMoveLeft(xMax, xMin, yMax, yMin) {
			curX = curX - 1
			if -curX > xMin {
				xMin = -curX
			}
		}
	}
	return absInt(curX) + absInt(curY)

}

func absInt(i int) int {
	if i < 0 {
		return -i
	} else {
		return i
	}

}
func shouldMoveRight(a int, b int, c int, d int) bool {
	return a == b && a == c && a == d
}

func shouldMoveUp(a int, b int, c int, d int) bool {
	return a > b && a > c && a > d
}

func shouldMoveLeft(xMax int, xMin int, yMax int, yMin int) bool {
	return xMax == yMax && xMax > xMin && xMax > yMin
}

func shouldMoveDown(xMax int, xMin int, yMax int, yMin int) bool {
	return xMax == xMin && xMax == yMax && xMax > yMin
}
