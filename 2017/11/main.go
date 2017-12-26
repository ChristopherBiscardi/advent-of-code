package main

import (
	"flag"
	"fmt"
	"log"
	"strings"
)

func main() {
	flag.Parse()
	for _, v := range flag.Args() {
		steps := strings.Split(v, ",")
		result := Process(steps)
		fmt.Printf("%d steps away", result)
	}
}

// x, y, and z are a 3d coordinate system on a tilted hexagonal grid
func Process(steps []string) int {
	x, y, z := 0, 0, 0
	for _, step := range steps {
		switch step {
		case "ne":
			x += 1
			z -= 1
		case "nw":
			x -= 1
			y += 1
		case "n":
			y += 1
			z -= 1
		case "se":
			x += 1
			y -= 1
		case "sw":
			x -= 1
			z += 1
		case "s":
			y -= 1
			z += 1
		default:
			log.Fatalf("%s is not a direction", step)
		}
	}

	if x+y+z != 0 {
		log.Fatalf("uhhh, calculations are off. %d + %d + %d should equal zero", x, y, z)
	}

	if x < 0 {
		x = -x
	}
	if y < 0 {
		y = -y
	}
	if z < 0 {
		z = -z
	}
	max := 0
	for _, v := range []int{x, y, z} {
		if v > max {
			max = v
		}
	}
	return max
}
func Collapse(m map[string]int, leftKey string, rightKey string) {
	left := m[leftKey]
	right := m[rightKey]
	if left > right {
		m[leftKey] = left - right
		m[rightKey] = 0
	} else if left < right {
		m[leftKey] = 0
		m[rightKey] = right - left
	} else {
		m[leftKey] = 0
		m[rightKey] = 0
	}
}
