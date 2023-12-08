use std::fs;
use std::ops::Range;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let lowest_location_individual = get_lowest_location_number(&contents, &parse_individual_seeds);
    //let lowest_location_ranges = get_lowest_location_number(&contents, &parse_seed_ranges);
    println!("part 1 {} part 2", lowest_location_individual);
}

fn parse_individual_seeds(seed_line: &str) -> Vec<i64> {
    seed_line
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn parse_seed_ranges(seed_line: &str) -> Vec<i64> {
    let numbers = seed_line
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    numbers
        .chunks(2)
        .map(|chunk| chunk[0]..(chunk[1] + chunk[0]))
        .flatten()
        .collect::<Vec<i64>>()
}

fn get_lowest_location_number(input: &str, seed_parser: &dyn Fn(&str) -> Vec<i64>) -> i64 {
    let data = parse_gardening_data(input, seed_parser);
    let location = data
        .seeds
        .iter()
        .map(|seed| {
            data.ranges
                .iter()
                .fold(*seed, |mapped, range| get_next(mapped, range))
        })
        .min()
        .unwrap();
    location
}

fn get_next(seed: i64, range: &Vec<(Range<i64>, i64)>) -> i64 {
    if let Some(range_for_seed) = range.iter().find(|r| r.0.contains(&seed)).to_owned() {
        return seed + range_for_seed.1;
    }
    return seed;
}

struct GardeningData {
    ranges: Vec<Vec<(Range<i64>, i64)>>,
    seeds: Vec<i64>,
}

fn parse_gardening_data(input: &str, seed_parser: &dyn Fn(&str) -> Vec<i64>) -> GardeningData {
    let blocks = input.split("\n\n").collect::<Vec<&str>>();
    let seeds = seed_parser(blocks[0]);
    let blocks = blocks[1..]
        .iter()
        .map(|x| x.split("\n").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let ranges = blocks
        .iter()
        .map(|block| {
            block[1..]
                .iter()
                .filter(|line| line != &&"")
                .map(|line| {
                    let numbers = line
                        .split_whitespace()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                    (
                        (numbers[1]..numbers[1] + numbers[2] + 1),
                        numbers[0] - numbers[1],
                    )
                })
                .collect::<Vec<(Range<i64>, i64)>>()
        })
        .collect::<Vec<Vec<(Range<i64>, i64)>>>();

    GardeningData { ranges, seeds }
}

#[test]
fn parsing_ranges() {
    let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48"#;
    let data = parse_gardening_data(input, &parse_individual_seeds);
    let ranges = data.ranges;

    assert_eq!(ranges, vec![vec![((98..101), -48), ((50..99), 2)]]);
    assert_eq!(data.seeds, vec![79, 14, 55, 13]);
}

#[test]
fn parsing_seeds_as_ranges() {
    let input = r#"seeds: 1 3 9 2

seed-to-soil map:
50 98 2
52 50 48"#;
    let data = parse_gardening_data(input, &parse_seed_ranges);

    assert_eq!(data.seeds, vec![1, 2, 3, 9, 10]);
}

#[test]
fn get_next_test() {
    let next = get_next(79, &vec![((98..101), -48), ((50..99), 2)]);
    assert_eq!(next, 81);
}

#[test]
fn parsing_multiple_blocks_of_ranges() {
    let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    let data = parse_gardening_data(input, &parse_individual_seeds);
    let ranges = data.ranges;

    assert_eq!(ranges.len(), 7)
}

#[test]
fn gets_the_lowest_location() {
    let input = r#"seeds: 10

seed-to-soil map:
52 10 48"#;
    let location = get_lowest_location_number(input, &parse_individual_seeds);
    assert_eq!(location, 52);
}

#[test]
fn example_works_part_1() {
    //  the destination range start, the source range start, and the range length.
    let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    let location_number = get_lowest_location_number(input, &parse_individual_seeds);
    assert_eq!(location_number, 35);
}

#[test]
fn example_works_part_2() {
    //  the destination range start, the source range start, and the range length.
    let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    let location_number = get_lowest_location_number(input, &parse_seed_ranges);
    assert_eq!(location_number, 46);
}
