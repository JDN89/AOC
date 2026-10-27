package day02_test

import (
	"testing"

	"aoc/day02"
)

func TestPart1(t *testing.T) {
	input := `
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
`

	got := day02.Part1(input)
	want := 2
	if got != want {
		t.Errorf(" {Part1() = %d result != want %d", got, want)
	}
}
