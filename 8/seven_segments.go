package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"sort"
	"strconv"
	"strings"
)

type Digit struct {
	SegmentCount int
	Value        int
}

var digits = []Digit{
	{6, 0}, //
	{2, 1},
	{5, 2}, //
	{5, 3}, //
	{4, 4},
	{5, 5}, //
	{6, 6}, //
	{3, 7},
	{7, 8},
	{6, 9}, //

	// 6 segments 0, 6, 9
}

func find(digits []Digit, predicate func(Digit) bool) Digit {
	for _, digit := range digits {
		if predicate(digit) {
			return digit
		}
	}
	return Digit{}
}

func contains(numbers []int, needle int) bool {
	for _, number := range numbers {
		if number == needle {
			return true
		}
	}
	return false
}

func sortString(in string) string {
	groupAsSlice := strings.Split(in, "")
	sort.Slice(groupAsSlice, func(a, b int) bool {
		return groupAsSlice[a] < groupAsSlice[b]
	})
	return strings.Join(groupAsSlice, "")
}

func ParseMappings(input []string) map[string]int {
	mappings := make(map[string]int)
	for _, wires := range input {
		if contains([]int{2, 3, 4, 7}, len(wires)) {
			withLength := func(d Digit) bool {
				return d.SegmentCount == len(wires)
			}
			mappings[sortString(wires)] = find(digits, withLength).Value
		}
	}
	return mappings
}

func ParseLine(line string) string {
	result := ""
	parts := strings.Split(line, "|")
	if len(parts) < 2 {
		return ""
		// should error
	}
	mappings := ParseMappings(strings.Split(parts[0], " "))
	outputSegments := strings.Split(strings.Trim(parts[1], " "), " ")
	for _, segments := range outputSegments {
		i, ok := mappings[sortString(segments)]
		if ok {
			result = result + strconv.Itoa(i)
		}
	}
	return result
}

func Parse(input string) []string {
	lines := strings.Split(input, "\n")
	found := make([]string, 0, len(lines))
	for _, line := range lines {
		found = append(found, ParseLine(line))
	}
	return found
}

func main() {
	content, err := ioutil.ReadFile("./input.txt")
	if err != nil {
		log.Fatal(err)
	}
	input := string(content)
	fmt.Println(len(Parse(input)))
}
