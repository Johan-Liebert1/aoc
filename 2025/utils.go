package main

import "fmt"

var GridNeighborsAdders = [][]int{
	{-1, -1},
	{-1, 0},
	{-1, 1},
	{1, -1},
	{1, 0},
	{1, 1},
	{0, -1},
	{0, 1},
}

func WithinBounds[T any](grid [][]T, row, col int) bool {
	return row >= 0 && col >= 0 && row < len(grid) && col < len(grid[0])
}

func Clone2D(src grid) grid {
	dst := make(grid, len(src))
	for i := range src {
		dst[i] = append([]rune(nil), src[i]...)
	}
	return dst
}

func PrintGrid(grid grid) {
	for _, row := range grid {
		for _, c := range row {
			fmt.Printf("%c ", c)
		}
		fmt.Println()
	}
}
