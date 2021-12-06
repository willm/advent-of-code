use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

fn get_points(line: Line) -> Vec<(i32, i32)> {
    let mut points: Vec<(i32, i32)> = vec![line.start, line.end];
    for x in cmp::min(line.start.0, line.end.0) + 1..cmp::max(line.end.0, line.start.0) {
        points.push((x, line.start.1));
    }
    for y in cmp::min(line.start.1, line.end.1) + 1..cmp::max(line.end.1, line.start.1) {
        points.push((line.start.0, y));
    }
    points
}

fn parse_coordinate(input: &str) -> (i32, i32) {
    let points: Vec<i32> = input
        .split(",")
        .map(|x| i32::from_str(x).unwrap())
        .collect();
    (points[0], points[1])
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
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let all_points: Vec<Vec<(i32, i32)>> = parse_lines(&contents)
        .into_iter()
        .filter(|line| is_horizontal_or_vertical(line))
        .map(|line| get_points(line))
        .collect();
    let overlapping_points = get_overlapping_points(all_points);
    println!("{:?}", overlapping_points.len());
    Ok(())
}

fn is_horizontal_or_vertical(line: &Line) -> bool {
    line.start.0 == line.end.0 || line.start.1 == line.end.1
}

fn get_overlapping_points(points: Vec<Vec<(i32, i32)>>) -> Vec<(i32, i32)> {
    let mut counts: HashMap<(i32, i32), i32> = HashMap::new();
    let flattened: Vec<(i32, i32)> = points.into_iter().flatten().collect();
    for point in flattened {
        if let Some(val) = counts.get_mut(&point) {
            *val = *val + 1;
        } else {
            counts.insert(point, 0);
        }
    }
    let mut overlaps: Vec<(i32, i32)> = vec![];
    for key in counts.keys() {
        if counts.get(key).unwrap() > &0 {
            overlaps.push(key.clone());
        }
    }
    overlaps.sort();
    overlaps
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

#[test]
fn test_get_points() {
    let line = Line {
        start: (0, 0),
        end: (0, 5),
    };
    let points = get_points(line);
    println!("{:?}", points);
    assert_eq!(points.len(), 6);
}

#[test]
fn test_get_points_2() {
    let mut x = [(0, 0), (0, 5)];
    x.sort();
    x.reverse();
    println!("{:?}", x);
    let line = Line {
        end: (0, 0),
        start: (0, 5),
    };
    let points = get_points(line);
    println!("{:?}", points);
    assert_eq!(points.len(), 6);
}

#[test]
fn test_get_points_backwards() {
    let line: Line = Line {
        start: (9, 4),
        end: (3, 4),
    };
    let mut points = get_points(line);
    points.sort();
    assert_eq!(points, vec![(3, 4), (4, 4), (5, 4), (6, 4), (7, 4), (8, 4)]);
}

#[test]
fn test_points_more() {
    let mut file = File::open("input-test.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let all_points: Vec<(i32, i32)> = parse_lines(&contents)
        .into_iter()
        .filter(|line| is_horizontal_or_vertical(line))
        .map(|line| get_points(line))
        .flatten()
        .filter(|x| x == &(4, 8))
        .collect();
    assert_eq!(all_points.len(), 2);
}

#[test]
fn test_overlapping_points() {
    let line_a = Line {
        start: (0, 0),
        end: (0, 5),
    };
    let line_b = Line {
        start: (0, 4),
        end: (0, 5),
    };
    let points = get_overlapping_points(vec![get_points(line_a), get_points(line_b)]);
    println!("{:?}", points);
    assert_eq!(points.len(), 2);
    assert_eq!(points[0], (0, 4));
    assert_eq!(points[1], (0, 5));
}

#[test]
fn test_overlapping_points_x() {
    let line_a = Line {
        start: (0, 0),
        end: (5, 0),
    };
    let line_b = Line {
        start: (4, 0),
        end: (5, 0),
    };
    let points = get_overlapping_points(vec![get_points(line_a), get_points(line_b)]);
    println!("{:?}", points);
    assert_eq!(points.len(), 2);
    assert_eq!(points[0], (4, 0));
    assert_eq!(points[1], (5, 0));
}

#[test]
fn test_horizontal_vertical() {
    assert_eq!(
        is_horizontal_or_vertical(&Line {
            start: (0, 0),
            end: (0, 4)
        }),
        true
    );
    assert_eq!(
        is_horizontal_or_vertical(&Line {
            start: (0, 0),
            end: (4, 0)
        }),
        true
    );
    assert_eq!(
        is_horizontal_or_vertical(&Line {
            start: (0, 0),
            end: (4, 4)
        }),
        false
    );
}
