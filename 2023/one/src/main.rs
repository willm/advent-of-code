use regex::Regex;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    println!("{}", part_two(&contents).unwrap());
}

fn part_two(input: &str) -> Result<i32, String> {
    let digit_names = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let lines = input.split("\n").collect::<Vec<&str>>();
    let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let reverse_re = Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    let calibration_sum = lines
        .iter()
        .map(|line| {
            let line_digits: Vec<i32> = re
                .captures_iter(line)
                .map(|c| {
                    let (x, [digit]) = c.extract();
                    println!("x is {:?}", x);
                    if let Ok(result) = digit.parse::<i32>() {
                        return result;
                    }
                    for digit_name in digit_names {
                        if digit_name.0 == digit {
                            return digit_name.1;
                        }
                    }
                    panic!("No digits found");
                })
                .collect();

            let reverse_line_digits: Vec<i32> = reverse_re
                .captures_iter(
                    String::from(*line)
                        .chars()
                        .rev()
                        .collect::<String>()
                        .as_ref(),
                )
                .map(|c| {
                    let (x, [digit]) = c.extract();
                    println!("x is {:?}", x);
                    if let Ok(result) = digit.parse::<i32>() {
                        return result;
                    }
                    for digit_name in digit_names {
                        if digit_name.0.chars().rev().collect::<String>() == digit {
                            return digit_name.1;
                        }
                    }
                    panic!("No digits found");
                })
                .collect();

            println!("{:?}", line_digits);

            if line_digits.len() < 1 {
                return 0;
            }
            let first_digit = line_digits.first().unwrap();
            let last_digit = reverse_line_digits.first().unwrap();
            println!("{}{}", first_digit, last_digit);
            format!("{}{}", first_digit, last_digit)
                .parse::<i32>()
                .unwrap()
        })
        .fold(0, |sum, i| sum + i);

    return Ok(calibration_sum);
}

fn get_result(input: &str) -> Result<i32, String> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let re = Regex::new(r"(\d)").unwrap();
    let calibration_sum = lines
        .iter()
        .map(|line| {
            let line_digits: Vec<i32> = re
                .captures_iter(line)
                .map(|c| {
                    let (_, [digit]) = c.extract();
                    digit.parse::<i32>().unwrap()
                })
                .collect();

            if line_digits.len() < 1 {
                return 0;
            }
            let first_digit = line_digits.first().unwrap();
            let last_digit = line_digits.last().unwrap();
            println!("{}{}", first_digit, last_digit);
            format!("{}{}", first_digit, last_digit)
                .parse::<i32>()
                .unwrap()
        })
        .fold(0, |sum, i| sum + i);

    return Ok(calibration_sum);
}

#[test]
fn example_works() {
    let input = "#1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet#";
    let result = 142;
    let answer = get_result(input).unwrap();
    assert_eq!(result, answer);
}

#[test]
fn part_2_example_works() {
    let input = "#two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen#";
    let result = 281;
    let answer = part_two(input).unwrap();
    assert_eq!(result, answer);
}

#[test]
fn part_2_example_bug() {
    let input = "6nbdzdlmqpdlgpcclc";
    let result = 66;
    let answer = part_two(input).unwrap();
    assert_eq!(result, answer);
}

#[test]
fn part_2_example_bug_2() {
    let input = "#ckmb52fldxkseven3fkjgcbzmnr7
gckhqpb6twoqnjxqplthree2fourkspnsnzxlz1#";
    let result = 118;
    let answer = part_two(input).unwrap();
    assert_eq!(result, answer);
}

#[test]
fn part_2_example_bug_3() {
    let input = "#8threesevenfourgbgteight5twonenjr";
    let result = 81;
    let answer = part_two(input).unwrap();
    assert_eq!(result, answer);
}

/*
#[test]
fn onig() {
    let re = onig::Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))").unwrap();
    let input = "8threesevenfourgbgteight5twonenjr";
    let cap = re.captures(input).unwrap().ennumerate();
    println!(cap);
    let result = 81;
    let answer = part_two(input).unwrap();
    assert_eq!(result, answer);
}
*/
