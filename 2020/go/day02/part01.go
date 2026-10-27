package day02

import (
	"fmt"
	"strings"
)

// 1-3 a: abcde
// 1-3 b: cdefg
// 2-9 c: ccccccccc
//
// Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.
//
// In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.

type Policy struct {
	min      int
	max      int
	letter   rune
	password string
}

// https://medium.com/@andreiboar/demystifying-golang-strings-05981b84f1a7
// https://go101.org/article/string.html
// https://go.dev/blog/strings
// https://pkg.go.dev/strings

// use stirng fields/cut/split
func Part1(input string) int {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	for index := range lines {
		fmt.Println(lines[index])
		line := lines[index]
		strings.ContainsAny
	}

	// TODO: reread strings and slices, convert each line to Policy struct, rest is simple

	// fmt.Println(lines)

	return 0
}
