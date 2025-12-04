package main

import (
	"fmt"
	"os"
	"strings"
)

const (
	Paper = '@'
	Empty = '.'
)

type grid [][]rune

func parseDay4() grid {
	fileR, _ := os.ReadFile("./inputs/day4.txt")
	file := string(fileR)

	grid := grid{}

	for lineRaw := range strings.SplitSeq(file, "\n") {
		line := strings.Trim(lineRaw, "\n \t")

		if len(line) == 0 {
			continue
		}

		row := []rune{}

		for _, char := range line {
			row = append(row, char)
		}

		grid = append(grid, row)
	}

	return grid
}

func findTotalAdjacentPaper(grid grid, row, col int) int {
	total := 0

	for _, val := range GridNeighborsAdders {
		nr, nc := row+val[0], col+val[1]

		if !WithinBounds(grid, nr, nc) {
			continue
		}

		if grid[nr][nc] == Paper {
			total += 1
		}
	}

	return total
}

func Day4P1() {
	grid := parseDay4()

	totalAccessible := 0

	for r := range grid {
		for c := range grid[0] {
			if grid[r][c] != Paper {
				continue
			}

			if findTotalAdjacentPaper(grid, r, c) < 4 {
				totalAccessible += 1
			}
		}
	}

	fmt.Println("Day4P1: ", totalAccessible)
}

func Day4P2() {
	grid := parseDay4()

	totalRemoved := 0

	for {
		removedIdx := [][]int{}

		for r := range grid {
			for c := range grid[0] {
				if grid[r][c] != Paper {
					continue
				}

				if findTotalAdjacentPaper(grid, r, c) < 4 {
					totalRemoved += 1
					removedIdx = append(removedIdx, []int{r, c})
				}
			}
		}

		if len(removedIdx) == 0 {
			break
		}

		for _, v := range removedIdx {
			grid[v[0]][v[1]] = Empty
		}
	}

	fmt.Println("Day4P2: ", totalRemoved)
}
