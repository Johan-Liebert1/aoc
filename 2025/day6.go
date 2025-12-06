package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func parseDay6() ([][]uint64, []string) {
	fileR, _ := os.ReadFile("./inputs/day6.txt")
	file := string(fileR)

	numbers := [][]uint64{}
	operators := []string{}

x:
	for line := range strings.SplitSeq(file, "\n") {
		numLine := []uint64{}

		for numStr := range strings.SplitSeq(line, " ") {
			numStr := strings.Trim(numStr, " \t\n")

			if len(numStr) == 0 {
				continue
			}

			num, err := strconv.ParseUint(numStr, 10, 64)

			// We're at the operators
			if err != nil {
				for opStr := range strings.SplitSeq(line, " ") {
					opStr := strings.Trim(opStr, " \t\n")

					if len(opStr) == 0 {
						continue
					}

					operators = append(operators, opStr)
				}

				break x
			}

			numLine = append(numLine, num)
		}

		numbers = append(numbers, numLine)
	}

	return numbers, operators
}

func Day6P1() {
	numbers, operators := parseDay6()

	var finalTotal uint64 = 0

	for col, op := range operators {
		thingies := []uint64{}

		for _, nRow := range numbers {
			thingies = append(thingies, nRow[col])
		}

		switch op {
		case "*":
			{
				var t uint64 = 1

				for _, n := range thingies {
					t *= n
				}

				finalTotal += t
			}

		case "+":
			{
				var t uint64 = 0

				for _, n := range thingies {
					t += n
				}

				finalTotal += t
			}

		default:
			{
				panic(fmt.Sprintf("Op %v unknown", op))
			}
		}
	}

	fmt.Println("Day6P1: ", finalTotal)
}
