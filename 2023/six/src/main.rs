use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let part_1 = get_margin_for_error(&contents);
    let part_2 = get_winning_options(&(42899189, 308117012911467)).len();
    println!("part 1 {} part 2 {}", part_1, part_2);
}

fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").filter(|s| *s != "").collect()
}

fn parse_line<T: AsRef<str>>(input: T) -> Vec<i64> {
    let foo = input.as_ref().replace("Time: ", "");
    let bar = foo.replace("Distance: ", "");
    bar.split_whitespace()
        .map(|v| v.parse::<i64>().unwrap())
        .collect()
}

fn parse(input: &str) -> Vec<(i64, i64)> {
    let lines: Vec<Vec<i64>> = split_lines(input).iter().map(parse_line).collect();
    lines
        .first()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, s)| (*s, lines[1][i]))
        .collect()
}

fn get_winning_options(race: &(i64, i64)) -> Vec<i64> {
    let (time, distance) = race;
    (0..*time)
        .filter(|button_time| {
            let moving_time = time - button_time;
            moving_time * button_time > *distance
        })
        .collect()
}

fn get_margin_for_error(input: &str) -> usize {
    let races = parse(input);
    races
        .iter()
        .map(get_winning_options)
        .map(|o| o.len())
        .product::<usize>()
}

#[test]
fn example_works() {
    let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
    let margin_for_error = get_margin_for_error(input);
    assert_eq!(margin_for_error, 288);
}

#[test]
fn parsing() {
    let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
    let parsed = parse(input);
    assert_eq!(parsed, vec![(7, 9), (15, 40), (30, 200)]);
}

#[test]
fn get_winning_options_test() {
    let race = (7, 9);
    let expected_winning_options = vec![2, 3, 4, 5];
    let winning_options = get_winning_options(&race);
    assert_eq!(winning_options, expected_winning_options);
}

#[test]
fn get_winning_options_test_two() {
    let race = (15, 40);
    let winning_options = get_winning_options(&race);
    assert_eq!(winning_options.len(), 8);
}

#[test]
fn get_winning_options_test_three() {
    let race = (30, 200);
    let winning_options = get_winning_options(&race);
    assert_eq!(winning_options.len(), 9);
}
