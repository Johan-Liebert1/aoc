package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Range struct {
	start uint64
	end   uint64
}

func getIds() []Range {
	fileR, _ := os.ReadFile("./inputs/day2.txt")
	file := string(fileR)

	vec := []Range{}

	for range1 := range strings.SplitSeq(file, ",") {
		r := strings.Split(strings.Trim(range1, "\n \t"), "-")

		start, _ := strconv.Atoi(r[0])
		end, _ := strconv.Atoi(r[1])

		vec = append(vec, Range{start: uint64(start), end: uint64(end)})
	}

	return vec
}

func isPalindromeKinda(x uint64) bool {
	str := strconv.Itoa(int(x))

	if len(str) % 2 != 0 {
		return false
	}

	i, j := 0, len(str) / 2

	for j < len(str) {
		if str[i] != str[j] {
			return false
		}

		i++
		j++
	}

	return  true
}

func Day2P1() {
	ranges := getIds()

	var total uint64 = 0

	for _, r := range ranges {
		for i := r.start; i <= r.end; i++ {
			if isPalindromeKinda(i) {
				total += i
			}
		}
	}

	fmt.Println("Day2 P1: ", total)
}

func RepeatsInSlice(strSlice string, toSearch string) bool {
	if len(strSlice) == 0 {
		return true
	}

	if len(strSlice) < len(toSearch) {
		return false
	}

	// the very next seq does not start with this seq
	// so it cannot be made up of this seq
	if toSearch != strSlice[:len(toSearch)] {
		return false
	}

	if len(strSlice) < len(toSearch) {
		return false
	}

	if len(strSlice) == len(toSearch) * 2 {
		if strSlice[len(toSearch):] != toSearch {
			return false
		}
	}

	return RepeatsInSlice(strSlice[len(toSearch):],toSearch)
}


func RepeatAtLeastTwoTimes(x uint64) bool {
	str := strconv.Itoa(int(x))

	for i := 1; i <= len(str) / 2; i++ {
		slice := str[0:i]
		if RepeatsInSlice(str[i:], slice) {
			return true
		}
	}

	return false
}

func Day2P2() {
	ranges := getIds()

	var total uint64 = 0

	for _, r := range ranges {
		for i := r.start; i <= r.end; i++ {
			if RepeatAtLeastTwoTimes(i) {
				total += i
			}
		}
	}

	fmt.Println("Day2 P2: ", total)
}
