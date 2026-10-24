package day01_test

import (
	"testing"

	"aoc/day01"
)

func TestPart1(t *testing.T) {
	input := `
1721
979
366
299
675
1456
`

	got := day01.Part1(input)
	want := 514579
	if got != want {
		t.Errorf(" {Part1() = %d result != want %d", got, want)
	}
}

func TestPart2(t *testing.T) {
	input := `
1721
979
366
299
675
1456
`

	got := day01.Part2(input)
	want := 241861950
	if got != want {
		t.Errorf(" {Part1() = %d result != want %d", got, want)
	}
}
