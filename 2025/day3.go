package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func parseDay3() []string {
	fileR, _ := os.ReadFile("./inputs/day3.txt")
	file := string(fileR)

	banks := []string{}

	for line := range strings.SplitSeq(file, "\n") {
		banks = append(banks, strings.Trim(line, "\n \t"))
	}

	return banks
}

func findLargestNum(start int, bank string) int {
	if start >= len(bank) {
		return 0
	}

	maxx := 0

	for i := start + 1; i < len(bank); i++ {
		newNumStr := string(bank[start]) + string(bank[i])
		newNum, _ := strconv.Atoi(newNumStr)

		if newNum > maxx {
			maxx = newNum
		}
	}

	return max(maxx, findLargestNum(start+1, bank))
}

func Day3P1() {
	banks := parseDay3()

	total := 0

	for _, bank := range banks {
		total += findLargestNum(0, bank)
	}

	fmt.Println("Day3P1: ", total)
}


// cache = [idx][num chars chosen]

// Can choose 12 numbers...
// bruh
//
// this shit's too slow, the possible states explode
func ___findLargestNum12(idx int, bank string, current string, cache map[string]int) int {
	key := strconv.Itoa(idx) + "-" + strconv.Itoa(len(current))

	if val, ok := cache[key]; ok {
		return  val
	}

	if len(current) == 12 {
		n, _ := strconv.Atoi(current)
		return n
	}

	if idx >= len(bank) {
		return 0
	}

	// choose the current idx
	chosen := ___findLargestNum12(idx+1, bank, current+string(bank[idx]), cache)

	// skip including the current idx
	skipped := ___findLargestNum12(idx+1, bank, current, cache)

	maxx := max(chosen, skipped)
	cache[key] = maxx

	return maxx
}

// check in windows of 12 digits
func largest12DigitsFaster(s string) int {
    numDigits := 12
    start := 0
    lenS := len(s)
    result := make([]byte, 0, numDigits)

    for numDigits > 0 {
        end := lenS - numDigits

        // Find the max digit in s[start : end + 1]
        maxDigit := byte('0')
        maxIndex := start

        for i := start; i <= end; i++ {
            if s[i] > maxDigit {
                maxDigit = s[i]
                maxIndex = i
                if maxDigit == '9' { // can't get better
                    break
                }
            }
        }

        result = append(result, maxDigit)
        start = maxIndex + 1
        numDigits--
    }

	largest, _ := strconv.Atoi(string(result))

    return largest
}

func Day3P2() {
	banks := parseDay3()

	total := 0

	for _, bank := range banks {
		total += largest12DigitsFaster(bank)
	}

	fmt.Println("Day3P2: ", total)
}
