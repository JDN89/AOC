package day01

import (
	"fmt"
	"strconv"
	"strings"
)

// BRUTE forcing the shit out of this. I think I am missing a clean solution
// That bein said, I'm surprised how easy it was for me to quickly jot this down. This C like syntax matches closer to my way of thinking

// They offer you a second one if you can find three numbers in your expense report that meet the same criteria.
//
// Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.
//
// In your expense report, what is the product of the three entries that sum to 2020?

func Part2(input string) int {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	fmt.Println(lines)

	bucket := [2020]bool{}

	result := 0
	// fmt.Println(len(lines))

	// loop over range and place all the values in the bucket = true
	// then when value encountered we know we found the right combination
	for z := range lines {
		entry, err := strconv.Atoi(lines[z])
		fmt.Printf("entryX:  %d \n", entry)
		if err != nil {
			fmt.Println("at index", z)
			panic(err)
		}
		bucket[entry] = true
	}

	for i := range lines {
		// fmt.Println(lines[i])

		entryX, err := strconv.Atoi(lines[i])
		fmt.Printf("entryX:  %d \n", entryX)
		if err != nil {
			fmt.Println("at index", i)
			panic(err)
		}

		remainder := 2020 - entryX
		if remainder >= 2020 || remainder < 0 {
			panic("this is not correct")
		}

		temp := 0
		for y := range lines {
			// reset temp
			temp = 0
			entryY, err := strconv.Atoi(lines[y])
			fmt.Printf("entryY:  %d \n", entryY)
			if err != nil {
				fmt.Println("at index", i)
				panic(err)
			}
			if entryY == entryX {
				fmt.Printf("X %d same as Y %d -- continue \n", entryX, entryY)
				continue
			}
			temp = remainder - entryY
			fmt.Printf("temp: %d \n", temp)

			if temp >= 2020 {
				panic(" temp >= 2020: this is not correct \n \n")
			}
			if temp < 0 {
				fmt.Println(" temp < 0: this is not correct")
				continue
			}
			if bucket[temp] == true {
				fmt.Printf("found true for entryX: %d, entryY %d and temp: %d \n\n", entryX, entryY, temp)
				result = entryY * entryX * temp
				break
			}
		}

		// bucket[temp] = true
		continue

	}
	return result
}

// CLEANER solutA cleaner solution is O(n²) using a hash set. For each number, solve the remaining problem as a two-sum search.
// Your current solution is also roughly O(n²), but it repeatedly parses input, uses a fixed-size bucket, and incorrectly skips equal values even when duplicates are valid.
// Pseudocode:
// numbers = parse input into integer list
// target = 2020
//
// for i from 0 to len(numbers) - 1:
//     first = numbers[i]
//     remaining = target - first
//
//     seen = empty set
//
//     for j from i + 1 to len(numbers) - 1:
//         second = numbers[j]
//         third = remaining - second
//
//         if third exists in seen:
//             return first * second * third
//
//         add second to seen
//
// return no solutionion according to AI (haven written this out)
