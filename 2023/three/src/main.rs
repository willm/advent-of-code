use core::ops::Range;
use regex::Regex;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let sum = sum_part_numbers(&contents);
    let gear_ratio = get_gear_ratio(&contents);
    println!("part 1 {} part 2 {}", sum, gear_ratio);
}

#[derive(Debug, PartialEq, Eq)]
struct NumberBlock {
    y: usize,
    x: Range<usize>,
    value: i32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Symbol {
    y: usize,
    x: usize,
    value: String,
}

fn split_lines(input: &str) -> Vec<&str> {
    input
        .split("\n")
        .filter(|s| *s != "")
        .collect::<Vec<&str>>()
}

fn parse_symbols(lines: &Vec<&str>) -> Vec<Symbol> {
    let is_symbol = Regex::new(r"([^\d\.])").unwrap();
    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            is_symbol
                .captures_iter(line)
                .map(|c| {
                    let m = c.get(1).unwrap();
                    Symbol {
                        y: i,
                        x: m.start(),
                        value: String::from(m.as_str()),
                    }
                })
                .collect::<Vec<Symbol>>()
        })
        .flatten()
        .collect()
}

fn parse_number_blocks(lines: &Vec<&str>) -> Vec<NumberBlock> {
    let is_number = Regex::new(r"(\d+)").unwrap();
    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            is_number
                .captures_iter(line)
                .map(|c| {
                    let m = c.get(1).unwrap();
                    NumberBlock {
                        y: i,
                        x: m.range(),
                        value: m.as_str().parse::<i32>().unwrap(),
                    }
                })
                .collect::<Vec<NumberBlock>>()
        })
        .flatten()
        .collect()
}

fn minus_one_clamp_zero(digit: usize) -> usize {
    if digit == 0 {
        return 0;
    }
    digit - 1
}

fn is_part(number_block: &NumberBlock, symbols: &Vec<Symbol>) -> bool {
    symbols.iter().any(|s| {
        let x_range = minus_one_clamp_zero(number_block.x.start)..(number_block.x.end + 1);
        let y_range = minus_one_clamp_zero(number_block.y)..(number_block.y + 2);
        x_range.contains(&s.x) && y_range.contains(&s.y)
    })
}

fn sum_part_numbers(input: &str) -> i32 {
    let lines = split_lines(input);
    let number_blocks = parse_number_blocks(&lines);
    let symbols = parse_symbols(&lines);
    number_blocks
        .iter()
        .filter(|n| is_part(n, &symbols))
        .fold(0, |sum, n| n.value + sum)
}

fn get_gear_ratio(input: &str) -> i32 {
    let lines = split_lines(input);
    let number_blocks = parse_number_blocks(&lines);
    let symbols = parse_symbols(&lines);
    symbols
        .iter()
        .filter(|s| s.value == "*")
        .map(|s| {
            number_blocks
                .iter()
                .filter(|n| is_part(n, &vec![s.clone()]))
                .map(|n| n.value)
                .collect::<Vec<i32>>()
        })
        .filter(|nbs| nbs.len() == 2)
        .map(|gear_parts| gear_parts.iter().fold(1, |product, value| product * value))
        .fold(0, |sum, n| n + sum)
}

#[test]
fn example_works() {
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598."#;
    let sum = sum_part_numbers(input);
    assert_eq!(sum, 4361);
}

#[test]
fn parse_a_block_of_numbers() {
    let input = "123";
    let blocks = parse_number_blocks(&vec![input]);
    assert_eq!(blocks.len(), 1);
    assert_eq!(
        blocks[0],
        NumberBlock {
            x: (0..3),
            y: 0,
            value: 123
        }
    );
}

#[test]
fn parse_multiple_blocks_of_numbers_on_the_same_line() {
    let input = "123....43";
    let blocks = parse_number_blocks(&vec![input]);
    assert_eq!(blocks.len(), 2);
    assert_eq!(
        blocks[0],
        NumberBlock {
            x: (0..3),
            y: 0,
            value: 123
        }
    );
    assert_eq!(
        blocks[1],
        NumberBlock {
            x: (7..9),
            y: 0,
            value: 43
        }
    );
}

#[test]
fn parse_multiple_number_blocks_on_multiple_lines() {
    let input = vec!["123....", "..43."];
    let blocks = parse_number_blocks(&input);
    assert_eq!(blocks.len(), 2);
    assert_eq!(
        blocks[0],
        NumberBlock {
            x: (0..3),
            y: 0,
            value: 123
        }
    );
    assert_eq!(
        blocks[1],
        NumberBlock {
            x: (2..4),
            y: 1,
            value: 43
        }
    );
}

#[test]
fn parse_a_symbol() {
    let input = "123.$...43";
    let symbols = parse_symbols(&vec![input]);
    assert_eq!(symbols.len(), 1);
    let symbol = symbols.get(0).unwrap();
    assert_eq!(
        symbol,
        &Symbol {
            x: 4,
            y: 0,
            value: String::from("$")
        }
    );
}

#[test]
fn parse_multiple_symbols() {
    let input = vec!["123.$...43", "...32%"];
    let symbols = parse_symbols(&input);
    assert_eq!(symbols.len(), 2);
    let symbol_1 = symbols.get(0).unwrap();
    assert_eq!(
        symbol_1,
        &Symbol {
            x: 4,
            y: 0,
            value: String::from("$")
        }
    );
    let symbol_2 = symbols.get(1).unwrap();
    assert_eq!(
        symbol_2,
        &Symbol {
            x: 5,
            y: 1,
            value: String::from("%")
        }
    );
}

#[test]
fn testing_if_a_number_block_is_a_part_symbol_to_the_left() {
    let symbols = vec![Symbol {
        x: 0,
        y: 0,
        value: String::from("*"),
    }];
    let number_block = NumberBlock {
        x: (1..4),
        y: 0,
        value: 123,
    };

    assert!(is_part(&number_block, &symbols));
}

#[test]
fn testing_if_a_number_block_is_a_part_symbol_to_the_right() {
    let symbols = vec![Symbol {
        x: 3,
        y: 0,
        value: String::from("*"),
    }];
    let number_block = NumberBlock {
        x: (0..3),
        y: 0,
        value: 123,
    };

    assert!(is_part(&number_block, &symbols));
}

#[test]
fn testing_if_a_number_block_is_a_part_symbol_below() {
    let symbols = vec![Symbol {
        x: 0,
        y: 1,
        value: String::from("*"),
    }];
    let number_block = NumberBlock {
        x: (0..3),
        y: 0,
        value: 123,
    };

    assert!(is_part(&number_block, &symbols));
}

#[test]
fn testing_if_a_number_block_is_a_part_symbol_above() {
    let symbols = vec![Symbol {
        x: 0,
        y: 1,
        value: String::from("*"),
    }];
    let number_block = NumberBlock {
        x: (0..3),
        y: 0,
        value: 123,
    };

    assert!(is_part(&number_block, &symbols));
}

#[test]
fn testing_if_a_number_block_is_a_part_symbol_diagonal_top_left() {
    let symbols = vec![Symbol {
        x: 0,
        y: 0,
        value: String::from("*"),
    }];
    let number_block = NumberBlock {
        x: (1..4),
        y: 1,
        value: 123,
    };

    assert!(is_part(&number_block, &symbols));
}

#[test]
fn testing_if_a_number_block_is_a_part_symbol_diagonal_top_right() {
    let symbols = vec![Symbol {
        x: 3,
        y: 0,
        value: String::from("*"),
    }];
    let number_block = NumberBlock {
        x: (0..3),
        y: 1,
        value: 123,
    };

    assert!(is_part(&number_block, &symbols));
}

#[test]
fn getting_gear_ratio() {
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    let ratio = get_gear_ratio(input);
    assert_eq!(ratio, 467835);
}
