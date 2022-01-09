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
	{6, 0},
	{2, 1},
	{5, 2},
	{5, 3},
	{4, 4},
	{5, 5},
	{6, 6},
	{3, 7},
	{7, 8},
	{6, 9},
}

func Find(digits []Digit, predicate func(Digit) bool) Digit {
	for _, digit := range digits {
		if predicate(digit) {
			return digit
		}
	}
	return Digit{}
}

func Contains(numbers []int, needle int) bool {
	for _, number := range numbers {
		if number == needle {
			return true
		}
	}
	return false
}

func SortString(in string) string {
	groupAsSlice := strings.Split(in, "")
	sort.Slice(groupAsSlice, func(a, b int) bool {
		return groupAsSlice[a] < groupAsSlice[b]
	})
	return strings.Join(groupAsSlice, "")
}

func KeyForValue(m map[string]int, needle int) (val string, ok bool) {
	for k, v := range m {
		if v == needle {
			return k, true
		}
	}
	return "", false
}

func Matches(haystack string, predicate func(string) bool) int {
	matches := 0
	chars := strings.Split(haystack, "")
	for _, char := range chars {
		if predicate(char) {
			matches += 1
		}
	}
	return matches
}

func ParseMappings(input []string) map[string]int {
	mappings := make(map[string]int)
	for _, wires := range input {
		if Contains([]int{2, 3, 4, 7}, len(wires)) {
			withLength := func(d Digit) bool {
				return d.SegmentCount == len(wires)
			}
			mappings[SortString(wires)] = Find(digits, withLength).Value
		}
	}
	for _, wires := range input {
		segments := SortString(wires)
		if len(wires) == 5 {
			mappings[segments], _ = Find5SegmentMapping(mappings, segments)
		}
		if len(wires) == 6 {
			mappings[segments], _ = Find6SegmentMapping(mappings, segments)
		}
	}
	return mappings
}

func Find5SegmentMapping(mappings map[string]int, segments string) (result int, ok bool) {
	seven, _ := KeyForValue(mappings, 7)
	inDigit := func(segment string) bool {
		return strings.Contains(segments, segment)
	}
	segmentsInSeven := Matches(seven, inDigit)

	if segmentsInSeven == 3 {
		return 3, true
	}
	four, _ := KeyForValue(mappings, 4)
	segmentsInFour := Matches(four, inDigit)

	if segmentsInFour == 2 {
		return 2, true
	}
	if segmentsInFour == 3 {
		return 5, true
	}
	return 0, false
}

func Find6SegmentMapping(mappings map[string]int, segments string) (result int, ok bool) {
	one, _ := KeyForValue(mappings, 1)
	inDigit := func(segment string) bool {
		return strings.Contains(segments, segment)
	}
	segmentsInOne := Matches(one, inDigit)
	if segmentsInOne == 1 {
		return 6, true
	}
	four, _ := KeyForValue(mappings, 4)
	segmentsInFour := Matches(four, inDigit)
	if segmentsInFour == 4 {
		return 9, true
	}
	return 0, false
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
		i, ok := mappings[SortString(segments)]
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
	rawValues := Parse(input)
	sum := int64(0)
	for _, reading := range rawValues {
		number, _ := strconv.ParseInt(reading, 10, 0)
		sum += number
	}
	fmt.Println(sum)
}
