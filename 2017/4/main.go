package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	sum := 0
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		if CheckPassphrase(scanner.Text()) {
			sum = sum + 1
		}
	}
	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		os.Exit(1)
	}
	fmt.Printf("%d valid passphrases", sum)

}

func CheckPassphrase(str string) bool {
	// empty struct requires no additional space. we care about keys
	var usedWords = make(map[string]struct{})
	for _, word := range strings.Fields(str) {
		_, exists := usedWords[word]
		if exists {
			// reused a word, fail.
			return false
		} else {
			usedWords[word] = struct{}{}
		}

	}
	return true
}
