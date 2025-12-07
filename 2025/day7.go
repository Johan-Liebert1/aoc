package main

import (
	"fmt"
	"os"
	"strings"
)

const (
	Start    = 'S'
	Beam     = '|'
	Dot      = '.'
	Splitter = '^'
)

func parseDay7() (grid, RowCol) {
	fileR, _ := os.ReadFile("./inputs/day7.txt")
	file := string(fileR)

	grid := grid{}

	rowCol := RowCol{}

	for row, line := range strings.Split(file, "\n") {
		line := strings.Trim(line, "\n \t")

		if len(line) == 0 {
			continue
		}

		rowList := []rune{}

		for col, char := range line {
			t := ' '

			switch char {
			case '.':
				t = Dot
			case 'S':
				t = Start
				rowCol = RowCol{row, col}
			case '^':
				t = Splitter
			case '|':
				t = Beam
			default:
				panic("Found weird char")
			}

			rowList = append(rowList, t)
		}

		grid = append(grid, rowList)
	}

	return grid, rowCol
}

func Day7P1() {
	grid, startRowCol := parseDay7()

	totalSplit := 0

	// Start beaming
	grid[startRowCol.row+1][startRowCol.col] = Beam

	beams := []RowCol{{row: startRowCol.row + 1, col: startRowCol.col}}

	for len(beams) > 0 {
		beam := beams[0]
		beams = beams[1:]

		nr := beam.row + 1
		nc := beam.col

		if !WithinBounds(grid, nr, nc) {
			continue
		}

		switch grid[nr][nc] {
		case Beam:
			continue
			// panic(fmt.Sprintf("Found beam. %d, %d", nr, nc))

		case Start:
			panic("Found start")

		case Dot:
			{
				grid[nr][nc] = Beam
				beams = append(beams, RowCol{row: nr, col: nc})
			}

		case Splitter:
			{
				if WithinBounds(grid, nr, nc-1) {
					grid[nr][nc-1] = Beam
					beams = append(beams, RowCol{row: nr, col: nc - 1})
				}

				if WithinBounds(grid, nr, nc+1) {
					grid[nr][nc+1] = Beam
					beams = append(beams, RowCol{row: nr, col: nc + 1})
				}

				totalSplit++
			}
		}
	}

	fmt.Println("Day7P1: ", totalSplit)
}
