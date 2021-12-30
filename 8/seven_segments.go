package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strings"
)

type digit struct {
	SegmentCount int
	Value        int
}

var digits = []digit{
	digit{2, 1},
	digit{4, 4},
	digit{3, 7},
	digit{7, 8},
}

func Parse(input string) []digit {
	lines := strings.Split(input, "\n")
	found := make([]digit, 0, 4)
	for _, line := range lines {
		parts := strings.Split(line, "|")
		if len(parts) < 2 {
			continue
		}
		segmentsGroups := strings.Split(strings.Trim(parts[1], " "), " ")
		for _, group := range segmentsGroups {
			for _, digit := range digits {
				if len(group) == digit.SegmentCount {
					found = append(found, digit)
				}
			}
		}
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
