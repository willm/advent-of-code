use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let steps = steps_to_zzz(&contents);
    println!("part 1 {}", steps);
}

#[derive(Debug)]
struct Place<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

fn steps_to_zzz(input: &str) -> usize {
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
