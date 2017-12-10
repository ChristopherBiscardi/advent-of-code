package main

import "testing"

func TestCheckPassphrase(t *testing.T) {
	WithPassphrase(t, "aa bb cc dd ee", true)
	WithPassphrase(t, "aa bb cc dd aa", false)
	WithPassphrase(t, "aa bb cc dd aaa", true)
}
func WithPassphrase(t *testing.T, str string, result bool) {
	isValidPassphrase := CheckPassphrase(str)
	if isValidPassphrase != result {
		t.Errorf("For %s, Expected %t, got %t", str, result, isValidPassphrase)
	}
}
