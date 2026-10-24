package aoc

import (
	"os"
	"strings"
)

func ReadInput(path string) string {
	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	// cast byte[] to string
	return strings.TrimSpace(string(data))
}

func ReadLines(path string) []string {
	return strings.Split(ReadInput(path), "\n")
}
