use num::integer::lcm;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let steps = steps_to_zzz(&contents);
    //let part_2 = steps_to_zzz_ghosts(&contents);
    let a: i64 = lcm(11911, 13019);
    let b = lcm(16343, 19667);
    let c = lcm(20221, 21883);
    let d = lcm(a, b);
    let part_2 = lcm(d, c);
    println!("part 1 {} part 2 {}", steps, part_2);
}

#[derive(Debug)]
struct Place<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

fn parse(input: &str) -> (Vec<char>, Vec<Place>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let directions: Vec<char> = parts[0].chars().collect();
    let places: Vec<Place> = parts[1]
        .split("\n")
        .filter(|s| s.ne(&""))
        .map(|line| Place {
            name: &line[0..3],
            left: &line[7..10],
            right: &line[12..15],
        })
        .collect();
    (directions, places)
}

fn steps_to_zzz_ghosts(input: &str) -> usize {
    let (directions, places) = parse(input);
    let mut steps: usize = 0;
    let mut positions: Vec<&Place> = places.iter().filter(|p| p.name.ends_with("A")).collect();

    while !positions.iter().all(|&p| p.name.ends_with("Z")) {
        let zspots: Vec<&&Place> = positions
            .iter()
            .filter(|&p| p.name.ends_with("Z"))
            .collect();
        if zspots.len() > 0 {
            println!("{:?} {}", zspots, steps);
        }
        let direction = directions[steps % directions.len()];
        steps = steps + 1;
        positions = positions
            .iter()
            .map(|place| {
                if direction == 'L' {
                    return places
                        .iter()
                        .find(|p| p.name.eq(place.left))
                        .expect("failed to find place");
                }
                if direction == 'R' {
                    return places.iter().find(|p| p.name.eq(place.right)).unwrap();
                }
                panic!("should never happen");
            })
            .collect();
    }
    steps
}

fn steps_to_zzz(input: &str) -> usize {
    let (directions, places) = parse(input);

    let mut steps: usize = 0;
    let mut place = places.first().expect("");
    while place.name != "ZZZ" {
        let direction = directions[steps % directions.len()];
        steps = steps + 1;
        if direction == 'L' {
            place = places
                .iter()
                .find(|p| p.name.eq(place.left))
                .expect("failed to find place")
        }
        if direction == 'R' {
            place = places.iter().find(|p| p.name.eq(place.right)).unwrap();
        }
    }
    steps
}

#[test]
fn example_works() {
    let input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    let steps = steps_to_zzz(input);
    assert_eq!(steps, 2);
}

#[test]
fn example_2_works() {
    let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
    let steps = steps_to_zzz_ghosts(input);
    assert_eq!(steps, 6);
}
