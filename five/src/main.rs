use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

fn parse_lines(input: &str) -> Vec<Line> {
    let line_regex = Regex::new(r"(?P<start>\d+,\d+) -> (?P<end>\d+,\d+)").unwrap();
    let mut lines: Vec<Line> = vec![];
    for capture in line_regex.captures_iter(input) {
        let start = parse_coordinate(&capture["start"]);
        let end = parse_coordinate(&capture["end"]);
        let line: Line = Line { start, end };
        lines.push(line)
    }
    lines
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input-test.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = parse_lines(&contents);
    println!("{:?}", lines);
    Ok(())
}

fn parse_coordinate(input: &str) -> (i32, i32) {
    let points: Vec<i32> = input
        .split(",")
        .map(|x| i32::from_str(x).unwrap())
        .collect();
    (points[0], points[1])
}

#[test]
fn test_parse_lines() {
    let lines = parse_lines("0,9 -> 5,9/n13,5 -> 0,0");
    assert_eq!(lines.len(), 2);

    assert_eq!(lines[0].start, (0, 9));
    assert_eq!(lines[0].end, (5, 9));

    assert_eq!(lines[1].start, (13, 5));
    assert_eq!(lines[1].end, (0, 0));
}
