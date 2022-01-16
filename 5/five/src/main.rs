use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Orientation {
    Horizonal,
    Vertical,
    Diagonal,
    Unknown,
}

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

impl Line {
    fn orientation(&self) -> Orientation {
        if is_horizontal(self) {
            return Orientation::Horizonal;
        } else if is_vertical(self) {
            return Orientation::Vertical;
        } else if is_45_deg(self) {
            return Orientation::Diagonal;
        }
        Orientation::Unknown
    }
}

fn get_points(line: Line) -> Vec<(i32, i32)> {
    let mut points: Vec<(i32, i32)> = vec![];
    match line.orientation() {
        Orientation::Horizonal | Orientation::Diagonal => {
            let x_diff = line.start.0 - line.end.0;
            let y_diff = line.start.1 - line.end.1;
            let gradient = y_diff / x_diff;
            for x in cmp::min(line.start.0, line.end.0)..cmp::max(line.end.0, line.start.0) + 1 {
                let y = (gradient * (x - line.start.0)) + line.start.1;
                points.push((x, y))
            }
        }
        Orientation::Vertical => {
            for y in cmp::min(line.start.1, line.end.1)..cmp::max(line.end.1, line.start.1) + 1 {
                points.push((line.start.0, y));
            }
        }
        Orientation::Unknown => panic!("unexpected orientation"),
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

fn number_of_overlapping_points(input_path: &str, orientations: Vec<Orientation>) -> usize {
    let mut file = File::open(input_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let all_points: Vec<Vec<(i32, i32)>> = parse_lines(&contents)
        .into_iter()
        .filter(|line| orientations.contains(&line.orientation()))
        .map(|line| get_points(line))
        .collect();
    let overlapping_points = get_overlapping_points(all_points);
    println!("{:?}", overlapping_points);
    overlapping_points.len()
}

fn main() -> std::io::Result<()> {
    println!(
        "{:?}",
        number_of_overlapping_points(
            "input.txt",
            vec![
                Orientation::Horizonal,
                Orientation::Vertical,
                Orientation::Diagonal
            ]
        )
    );
    Ok(())
}

fn is_horizontal(line: &Line) -> bool {
    line.start.1 == line.end.1
}

fn is_vertical(line: &Line) -> bool {
    line.start.0 == line.end.0
}

fn is_45_deg(line: &Line) -> bool {
    cmp::max(line.end.1, line.start.1) - cmp::min(line.start.1, line.end.1)
        == cmp::max(line.end.0, line.start.0) - cmp::min(line.start.0, line.end.0)
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
    assert_eq!(lines[0].orientation(), Orientation::Horizonal);

    assert_eq!(lines[1].start, (13, 5));
    assert_eq!(lines[1].end, (0, 0));
    assert_eq!(lines[1].orientation(), Orientation::Unknown);
}

#[test]
fn test_get_points_diagonal() {
    let line = Line {
        start: (1, 1),
        end: (3, 3),
    };
    assert_eq!(line.orientation(), Orientation::Diagonal);
    let mut points = get_points(line);
    points.sort();
    println!("{:?}", points);
    assert_eq!(points, [(1, 1), (2, 2), (3, 3)]);
}

#[test]
fn test_get_points_diagonal_2() {
    let line = Line {
        start: (8, 0),
        end: (6, 2),
    };
    let mut points = get_points(line);
    points.sort();
    println!("{:?}", points);
    assert_eq!(points, [(6, 2), (7, 1), (8, 0)]);
}

#[test]
fn test_get_points_diagonal_3() {
    let line = Line {
        start: (9, 7),
        end: (7, 9),
    };

    let mut points = get_points(line);
    points.sort();
    println!("{:?}", points);
    assert_eq!(points, [(7, 9), (8, 8), (9, 7)]);
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
    assert_eq!(
        points,
        vec![(3, 4), (4, 4), (5, 4), (6, 4), (7, 4), (8, 4), (9, 4)]
    );
}

#[test]
fn test_integration_test_part_1() {
    assert_eq!(
        number_of_overlapping_points(
            "input-test.txt",
            vec![Orientation::Horizonal, Orientation::Vertical]
        ),
        5
    );
}

#[test]
fn test_integration_test_part_2() {
    assert_eq!(
        number_of_overlapping_points(
            "input-test.txt",
            vec![
                Orientation::Horizonal,
                Orientation::Vertical,
                Orientation::Diagonal
            ]
        ),
        12
    );
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
        &Line {
            start: (0, 0),
            end: (0, 4)
        }
        .orientation(),
        &Orientation::Vertical
    );
    assert_eq!(
        &Line {
            start: (0, 0),
            end: (4, 0)
        }
        .orientation(),
        &Orientation::Horizonal
    );
    assert_eq!(
        &Line {
            start: (0, 0),
            end: (4, 4)
        }
        .orientation(),
        &Orientation::Diagonal
    );
    assert_eq!(
        &Line {
            start: (1, 1),
            end: (3, 3)
        }
        .orientation(),
        &Orientation::Diagonal
    );
    assert_eq!(
        &Line {
            start: (9, 7),
            end: (7, 9)
        }
        .orientation(),
        &Orientation::Diagonal
    );
}
