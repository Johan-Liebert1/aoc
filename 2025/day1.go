package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

// L68
// L30
// R48
// L5
// R60
func getData() []int {
	fileR, _ := os.ReadFile("./inputs/day1.txt")
	file := string(fileR)

	rotations := []int{}

	for _, rawLine := range strings.Split(file, "\n") {
		line := strings.Trim(rawLine, "\n \t")

		if len(line) == 0 {
			break
		}

		num, err := strconv.Atoi(line[1:])

		if err != nil {
			panic(err)
		}

		if line[0] == 'L' {
			num *= -1
		}

		rotations = append(rotations, num)
	}

	return rotations
}

func Day1p1() {
	data := getData()

	current := 50
	count := 0

	for _, d := range data {
		if current == 0 {
			count++
		}

		current = (current + d + 100) % 100
	}

	fmt.Println("Day1P1: ", count)
}

func Day1p2() {
	data := getData()

	current := 50
	count := 0

	for _, d := range data {
		dd := d

		add := 1

		if d > 0 {
			add = -1
		}

		for dd != 0 {
			if current == 0 {
				count++
			}

			current += -add
			dd += add

			if current < 0 {
				current = 99
			} else if current > 99 {
				current = 0
			}
		}
	}

	fmt.Println("Day1P2: ", count)
}
