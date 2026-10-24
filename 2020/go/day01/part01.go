package day01

import (
	"fmt"
	"strconv"
	"strings"
)

// REFERENCES:
// https://www.bytesizego.com/blog/reading-file-line-by-line-golang
// https://www.bytesizego.com/blog/set-in-golang
// convert to number with strconv.Atoi  https://pkg.go.dev/strconv#Atoi

// TODO: profile: https://go.dev/blog/pprof

// Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.

// iterate over lines
// bigger then 2020 early return
// bucket[2020]
// idx := 2020 - ipnut
// bucket[idx] != nill -> multiply
// else insert in bucket

func Part1(input string) int {
	lines := strings.Split(strings.TrimSpace(input), "\n")

	bucket := [2020]bool{}

	result := 0
	// fmt.Println(len(lines))
	for i := range lines {
		// fmt.Println(lines[i])

		entry, err := strconv.Atoi(lines[i])
		if err != nil {
			fmt.Println("at index", i)
			panic(err)
		}

		remainder := 2020 - entry

		if remainder >= 2020 || remainder < 0 {
			panic("this is not correct")
		}
		if bucket[remainder] == true {
			fmt.Println("found result")
			result = remainder * entry
			break
		}

		bucket[entry] = true

	}
	return result
}
