use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let value = get_total_value(&contents);
    let total = get_total_scratchcards(&contents);
    println!("value {} total {}", value, total);
}

fn split_lines(input: &str) -> Vec<&str> {
    input.split("\n").filter(|s| *s != "").collect()
}

fn get_total_value(input: &str) -> usize {
    split_lines(input)
        .iter()
        .map(parse_line)
        .map(get_score)
        .sum::<usize>()
}

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    my_numbers: Vec<i32>,
    winning_numbers: Vec<i32>,
}

fn parse_line<T: AsRef<str>>(input: T) -> Card {
    let id_data = input.as_ref().split(":").collect::<Vec<&str>>();
    let id = id_data[0]
        .replace("Card ", "")
        .trim()
        .parse::<u32>()
        .unwrap();
    let data = id_data[1];
    let sets = data
        .split("|")
        .map(|s| {
            s.trim()
                .split(" ")
                .filter(|s| s != &"")
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    Card {
        winning_numbers: sets.get(0).unwrap().clone(),
        my_numbers: sets.get(1).unwrap().clone(),
        id: id.try_into().unwrap(),
    }
}

fn get_score(card: Card) -> usize {
    let winning_numbers = card.winning_numbers;
    let my_numbers = card.my_numbers;
    my_numbers
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .fold(0, |points, _n| {
            if points == 0 {
                return 1;
            }
            return points * 2;
        })
}

fn get_total_scratchcards(input: &str) -> usize {
    let initial_cards = split_lines(input)
        .iter()
        .map(parse_line)
        .collect::<Vec<Card>>();
    let mut won_cards: Vec<&Card> = initial_cards.iter().collect::<Vec<&Card>>();
    let mut total = 0;
    while let Some(card) = won_cards.pop() {
        total += 1;
        let score = card
            .my_numbers
            .iter()
            .filter(|n| card.winning_numbers.contains(n))
            .count();
        initial_cards
            .iter()
            .filter(|c| c.id > card.id && c.id <= card.id + score)
            .for_each(|c| won_cards.push(c));
    }

    println!("{:?}", total);

    total
}

#[test]
fn example_works() {
    let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    let total_value = get_total_value(input);
    assert_eq!(total_value, 13);
}

#[test]
fn parsing_a_line() {
    let raw_line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    let expected = (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]);
    let parsed = parse_line(raw_line);
    assert_eq!(parsed.winning_numbers, expected.0);
    assert_eq!(parsed.my_numbers, expected.1);
    assert_eq!(parsed.id, 1);
}

#[test]
fn getting_line_score() {
    let line = Card {
        winning_numbers: vec![41, 48, 83, 86, 17],
        my_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        id: 0,
    };
    let score = get_score(line);
    assert_eq!(score, 8);
}

#[test]
fn part_2_simple() {
    let input = r#"Card 1: 1 | 1
Card 2: 3 | 61"#;
    let total = get_total_scratchcards(input);
    assert_eq!(total, 3);
}

#[test]
fn part_2_example() {
    let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    let total = get_total_scratchcards(input);
    assert_eq!(total, 30);
}
