package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
)

var (
	reg       = regexp.MustCompile("^([a-z]*) (inc|dec) ([0-9\\-]*) if ([a-z]*) ([!<>=]*) ([0-9\\-]*)")
	registers = make(map[string]int)
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		match := parseMatch(scanner.Text())
		//		fmt.Println("match", match)
		Iterate(registers, match)
	}
	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		os.Exit(1)
	}

	max := 0
	for k := range registers {
		//		fmt.Printf("%d,", registers[k])
		if registers[k] > max {
			max = registers[k]
		}
	}
	fmt.Printf("max is %d", max)
}

func Iterate(rMap map[string]int, match Match) {
	i, exists := rMap[match.TestName]
	if !exists {
		i = 0
	}
	if ShouldOperate(i, match.TestOp, match.TestInt) {
		val, registerExists := rMap[match.Name]
		if registerExists {
			//			fmt.Printf("%d %s %d\n", val, match.Operation, match.OpNumber)
			rMap[match.Name] = Operate(val, match.Operation, match.OpNumber)
		} else {
			//			fmt.Printf("%d %s %d\n", 0, match.Operation, match.OpNumber)
			rMap[match.Name] = Operate(0, match.Operation, match.OpNumber)
		}
	}

}
func Operate(oldValue int, op string, val int) int {
	switch op {
	case "inc":
		return oldValue + val
	case "dec":
		return oldValue - val
	default:
		log.Fatal("expected `inc` or `dec`, not %s", op)
		return -1
	}
}

func ShouldOperate(i int, op string, rhsI int) bool {
	//	fmt.Printf("ShouldOperate `%d %s %d`", i, op, rhsI)
	switch op {
	case ">":
		return i > rhsI
	case "<":
		return i < rhsI
	case ">=":
		return i >= rhsI
	case "<=":
		return i <= rhsI
	case "==":
		return i == rhsI
	case "!=":
		return i != rhsI
	default:
		log.Fatalf("invalid operation `%s`", op)
		return false
	}
}

func parseMatch(str string) Match {
	m := reg.FindAllStringSubmatch(str, -1)[0]
	opNum, err := strconv.Atoi(m[3])
	if err != nil {
		log.Fatal("Expected m[3] to be a number, got %d", m[3])
	}
	testNum, err := strconv.Atoi(m[6])
	if err != nil {
		log.Fatal("Expected m[6] to be a number, got %d", m[6])
	}
	return Match{m[1], m[2], opNum, m[4], m[5], testNum}
}

type Match struct {
	Name      string
	Operation string
	OpNumber  int
	TestName  string
	TestOp    string
	TestInt   int
}
