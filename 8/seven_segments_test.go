package main

import (
	"fmt"
	"testing"
)

/*func TestOneLine(t *testing.T) {
	input := [4]string{"fdgacbe", "cefdb", "cefbgd", "gcbe"}
	output := GetLineOutput(input)

	if output != "8394" {
		t.Errorf("Wrong output value %s expected %s", output, "8394")
	}
}*/

/*func GetLineOutput(input [4]string) string {
	found := make([]string, 0, 4)
	for _, group := range input {
		groupAsSlice := strings.Split(group, "")
		sort.Slice(groupAsSlice, func(a, b int) bool {
			return groupAsSlice[a] < groupAsSlice[b]
		})
		for _, digit := range digits {
			if strings.Join(groupAsSlice, "") == digit.Segments {
				found = append(found, strconv.Itoa(digit.Value))
			}
		}
	}
	return strings.Join(found, "")
}*/

func TestParseMappings(t *testing.T) {
	input := []string{"acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab"}
	mappings := ParseMappings(input)

	if mappings["abcdefg"] != 8 {
		t.Errorf("abcdefg should equal 8 was %d", mappings["abcdefg"])
	}

	if mappings["abd"] != 7 {
		t.Errorf("abd should equal 7 was %d", mappings["abd"])
	}

	if mappings["abef"] != 4 {
		t.Errorf("abd should equal 4 was %d", mappings["abef"])
	}

	if mappings["ab"] != 1 {
		t.Errorf("abd should equal 1 was %d", mappings["ab"])
	}

	if mappings["bcdef"] != 5 {
		t.Errorf("abd should equal 5 was %d", mappings["bcdef"])
	}
}

func TestParseLine(t *testing.T) {
	line := "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
	output := ParseLine(line)
	if output != "84" {
		t.Errorf("expected ouput to be 84 was %s", output)
	}
}

func TestPart1(t *testing.T) {
	input := "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fdgacbe cefdb cefbgd gcbe"
	parsed := Parse(input)

	if parsed[0] != "84" {
		t.Errorf("expected 84 got %s", parsed[0])
	}
	fmt.Printf("%s", parsed)
	if len(parsed) != 2 {
		t.Errorf("Expected length 4 got %d", len(parsed))
	}
}
