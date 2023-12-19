use std::fs;
fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    println!(
        "part 1 {} part 2 {}",
        get_sum_next_value(&contents),
        get_sum_previous_value(&contents)
    );
}

fn get_sum_next_value(input: &str) -> i32 {
    let parsed: Vec<Vec<i32>> = split_lines(input)
        .iter()
        .map(|l| {
            l.split(" ")
                .map(|d| d.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    parsed
        .iter()
        .map(|l| get_sum_next_value_line(l.clone()))
        .sum()
}

fn get_sum_previous_value(input: &str) -> i32 {
    let parsed: Vec<Vec<i32>> = split_lines(input)
        .iter()
        .map(|l| {
            l.split(" ")
                .map(|d| d.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    parsed
        .iter()
        .map(|l| get_sum_previous_value_line(l.clone()))
        .sum()
}

struct Diff {
    cur: Vec<i32>,
    initial: Option<Vec<i32>>,
}
impl Iterator for Diff {
    type Item = Vec<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(init) = self.initial.clone() {
            self.initial = None;
            return Some(init);
        }
        if self.cur.iter().all(|&x| x == 0) {
            return None;
        }
        let diffs = self
            .cur
            .iter()
            .enumerate()
            .fold(vec![], |mut diffs, (i, _x)| {
                if i == self.cur.len() - 1 {
                    return diffs;
                }
                diffs.push(self.cur[i + 1] - self.cur[i]);
                diffs
            });
        self.cur = diffs.clone();
        Some(diffs)
    }
}

fn diffs(list: Vec<i32>) -> Diff {
    Diff {
        initial: Some(list.clone()),
        cur: list,
    }
}

fn get_sum_next_value_line(line: Vec<i32>) -> i32 {
    let mut differences: Vec<Vec<i32>> = diffs(line).collect();
    for x in 0..differences.len() {
        let i = differences.len() - x - 1;
        if i == differences.len() - 1 {
            differences[i].push(0);
        } else {
            //let mut current_row = differences[i];
            let last_item_index = differences[i].len() - 1;
            let item_to_the_left = differences[i][last_item_index];
            let item_below = differences[i + 1][last_item_index];
            differences[i].push(item_to_the_left + item_below);
        }
    }
    *differences.first().unwrap().last().unwrap()
}

fn get_sum_previous_value_line(line: Vec<i32>) -> i32 {
    let mut differences: Vec<Vec<i32>> = diffs(line)
        .map(|mut line| {
            line.reverse();
            line
        })
        .collect();
    for x in 0..differences.len() {
        let i = differences.len() - x - 1;
        if i == differences.len() - 1 {
            differences[i].push(0);
        } else {
            //let mut current_row = differences[i];
            let last_item_index = differences[i].len() - 1;
            let item_to_the_left = differences[i][last_item_index];
            let item_below = differences[i + 1][last_item_index];
            differences[i].push(item_to_the_left - item_below);
        }
    }
    *differences.first().unwrap().last().unwrap()
}

fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").filter(|s| *s != "").collect()
}

#[test]
fn example_works() {
    let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;
    let sum_next_value = get_sum_next_value(input);
    assert_eq!(sum_next_value, 114);
}

#[test]
fn diffs_for_a_single_line() {
    let input = vec![0, 3, 6, 9, 12, 15];
    let differences: Vec<Vec<i32>> = diffs(input).collect();
    assert_eq!(
        differences,
        vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![3, 3, 3, 3, 3],
            vec![0, 0, 0, 0]
        ]
    );
}

#[test]
fn diffs_for_another_single_line() {
    let input = vec![1, 3, 6, 10, 15, 21];
    let differences: Vec<Vec<i32>> = diffs(input).collect();
    assert_eq!(
        differences,
        vec![
            vec![1, 3, 6, 10, 15, 21],
            vec![2, 3, 4, 5, 6],
            vec![1, 1, 1, 1],
            vec![0, 0, 0]
        ]
    );
}

#[test]
fn a_single_line() {
    let input = vec![0, 3, 6, 9, 12, 15];
    let sum_next_value = get_sum_next_value_line(input);
    assert_eq!(sum_next_value, 18);
}

#[test]
fn another_single_line() {
    let input = vec![1, 3, 6, 10, 15, 21];
    let sum_next_value = get_sum_next_value_line(input);
    assert_eq!(sum_next_value, 28);
}

#[test]
fn a_single_line_with_more_steps() {
    let input = vec![10, 13, 16, 21, 30, 45];
    let sum_next_value = get_sum_next_value_line(input);
    assert_eq!(sum_next_value, 68);
}

#[test]
fn previous_a_single_line() {
    let input = vec![10, 13, 16, 21, 30, 45];
    let sum_next_value = get_sum_previous_value_line(input);
    assert_eq!(sum_next_value, 5);
}
