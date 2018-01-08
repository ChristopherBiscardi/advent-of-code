package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	input := make(map[int]Segment)
	max := 0
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		split := strings.Split(scanner.Text(), ": ")
		keyDigit, err := strconv.Atoi(split[0])
		if err != nil {
			log.Fatal(err)
		}

		valueDigit, err := strconv.Atoi(split[0])
		if err != nil {
			log.Fatal(err)
		}

		if keyDigit > max {
			max = keyDigit
		}
		input[keyDigit] = NewSegment(valueDigit)
	}
	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		os.Exit(1)
	}

	fmt.Printf("%d members\n", Process(input, max))
}

type Segment struct {
	Length  int
	Value   int
	Reverse bool
}

func NewSegment(length int) Segment {
	return Segment{length, 0, false}
}

func (s *Segment) SimulateStep() {
	if s.Value == 0 && s.Reverse {
		s.Reverse = false
		s.Value++
	} else if !s.Reverse && s.Value < s.Length {
		if s.Value+1 >= s.Length {
			s.Value--
			s.Reverse = true
		} else {
			s.Value++
		}
	} else {
		s.Value--
	}
}

func SimulateStepMap(m map[int]Segment) {
	for k, v := range m {
		v.SimulateStep()
		m[k] = v
	}
}

func Process(input map[int]Segment, max int) int {
	sum := 0
	for i := 0; i <= max; i++ {
		if segment, ok := input[i]; ok {
			sum += GetSeverity(i, segment)
		}
		SimulateStepMap(input)
	}
	return sum
}

func GetSeverity(index int, s Segment) int {
	if s.Value == 0 {
		return index * s.Length
	}
	return 0
}
