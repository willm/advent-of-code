package main

import (
	"testing"
)

func TestPart1(t *testing.T) {
	input := "fio | fdgacbe cefdb cefbgd gcbe\nfio | fdgacbe cefdb cefbgd gcbe"
	parsed := Parse(input)
	if parsed[0].SegmentCount != 7 {
		t.Errorf("expected 7 got %d", parsed[0].SegmentCount)
	}
	if len(parsed) != 4 {
		t.Errorf("Expected length 4 got %d", len(parsed))
	}
}
