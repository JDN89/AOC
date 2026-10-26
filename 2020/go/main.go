package main

import (
	"fmt"
	"os"

	"aoc/day01"
)

func main() {
	if len(os.Args) < 2 {
		panic("usage: go run . <day>")
	}
	day := os.Args[1]

	input, err := os.ReadFile("day" + day + "/input.txt")
	if err != nil {
		panic(err)
	}

	d1p1 := day01.Part1(string(input))
	d1p2 := day01.Part2(string(input))

	fmt.Println(d1p1)
	fmt.Println(d1p2)
}
