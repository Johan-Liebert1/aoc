package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func parseDay5() ([]Range, []uint64) {
	fileR, _ := os.ReadFile("./inputs/day5.txt")
	file := string(fileR)

	split := strings.Split(file, "\n\n")
	ranges, idsStr := split[0], split[1]

	actualRanges := []Range{}

	for r := range strings.SplitSeq(ranges, "\n") {
		s := strings.Split(r, "-")
		start, _ := strconv.ParseUint(s[0], 10, 64)
		end, _ := strconv.ParseUint(s[1], 10, 64)

		actualRanges = append(actualRanges, Range{start, end})
	}

	ids := []uint64{}

	for i := range strings.SplitSeq(idsStr, "\n") {
		ii, _ := strconv.ParseUint(i, 10, 64)
		ids = append(ids, ii)
	}

	return actualRanges, ids
}

func Day5P1() {
	ranges, ids := parseDay5()

	total := 0

	for _, id := range ids {
		for _, r := range ranges {
			if id >= uint64(r.start) && id <= uint64(r.end) {
				total++
				break
			}
		}
	}

	fmt.Printf("Day5P1: %d\n", total)
}

func Day5P2() {
	goodRanges, _ := parseDay5()

	sort.Slice(goodRanges, func(i, j int) bool {
		return goodRanges[i].start < goodRanges[j].start
	})

	merged := []Range{goodRanges[0]}

	for i := 1; i < len(goodRanges); i++ {
		if goodRanges[i].start <= merged[len(merged)-1].end {
			merged[len(merged)-1].end = max(goodRanges[i].end, merged[len(merged)-1].end)
		} else {
			merged = append(merged, goodRanges[i])
		}
	}

	var total uint64 = 0

	for _, r := range merged {
		total += r.end - r.start + 1
	}

	fmt.Println("Day5P2: ", total)
}
