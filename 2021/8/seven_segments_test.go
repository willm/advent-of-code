package main

import (
	"fmt"
	"strings"
	"testing"
)

func TestMatches(t *testing.T) {
	allInX := func(segment string) bool {
		return strings.Contains("abcd", segment)
	}
	result := Matches("abc", allInX)
	if result != 3 {
		t.Errorf("abcd all in abc")
	}
}

func TestParseMappings(t *testing.T) {
	input := []string{"acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab"}
	mappings := ParseMappings(input)

	fmt.Println(mappings)

	if mappings["abcdeg"] != 0 {
		t.Errorf("abcdeg should equal 0 was %d", mappings["abcdeg"])
	}

	if mappings["ab"] != 1 {
		t.Errorf("abd should equal 1 was %d", mappings["ab"])
	}

	if mappings["acdfg"] != 2 {
		t.Errorf("abd should equal 2 was %d", mappings["acdfg"])
	}

	if mappings["abcdf"] != 3 {
		t.Errorf("abd should equal 3 was %d", mappings["abcdf"])
	}

	if mappings["abef"] != 4 {
		t.Errorf("abd should equal 4 was %d", mappings["abef"])
	}

	if mappings["bcdef"] != 5 {
		t.Errorf("abd should equal 5 was %d", mappings["bcdef"])
	}

	if mappings["bcdefg"] != 6 {
		t.Errorf("bcdefg should equal 6 was %d", mappings["bcdefg"])
	}

	if mappings["abd"] != 7 {
		t.Errorf("abd should equal 7 was %d", mappings["abd"])
	}

	if mappings["abcdefg"] != 8 {
		t.Errorf("abcdefg should equal 8 was %d", mappings["abcdefg"])
	}

	if mappings["abcdef"] != 9 {
		t.Errorf("abcdef should equal 9 was %d", mappings["abcdef"])

	}
}

func TestParseLine(t *testing.T) {
	line := "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
	output := ParseLine(line)
	if output != "8394" {
		t.Errorf("expected ouput to be 84 was %s", output)
	}
}

func TestPart1(t *testing.T) {
	input := "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fdgacbe cefdb cefbgd gcbe"
	parsed := Parse(input)

	if parsed[0] != "8394" {
		t.Errorf("expected 84 got %s", parsed[0])
	}
	fmt.Printf("%s", parsed)
	if len(parsed) != 2 {
		t.Errorf("Expected length 4 got %d", len(parsed))
	}
}
