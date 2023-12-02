use std::cmp;
use std::fs;
fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let impossible = sum_impossible(&contents);
    let min = sum_minimum(&contents);
    println!("impossible: {}, minimum: {}", impossible, min);
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: i32,
    blue: i32,
    green: i32,
    red: i32,
}

impl Game {
    fn new(line: &str) -> Self {
        let id_sets = line.split(": ").collect::<Vec<&str>>();
        let id = id_sets[0].replace("Game ", "").parse::<i32>().unwrap();
        let sets = id_sets[1].split("; ");
        let set_cubes = sets.map(|set| set.split(", ")).flatten();
        let rgb = set_cubes
            .map(|x| {
                let s = x.split(" ").collect::<Vec<&str>>();
                (s[0].parse::<i32>().unwrap(), s[1])
            })
            .fold((0, 0, 0), |rgb, cur| {
                if cur.1 == "red" {
                    return (cmp::max(rgb.0, cur.0), rgb.1, rgb.2);
                }
                if cur.1 == "green" {
                    return (rgb.0, cmp::max(rgb.1, cur.0), rgb.2);
                }
                if cur.1 == "blue" {
                    return (rgb.0, rgb.1, cmp::max(rgb.2, cur.0));
                }
                rgb
            });

        Game {
            id,
            blue: rgb.2,
            green: rgb.1,
            red: rgb.0,
        }
    }
}

fn sum_minimum(input: &str) -> i32 {
    input
        .split("\n")
        .filter(|s| *s != "")
        .map(Game::new)
        .fold(0, |sum, game| sum + game.red * game.green * game.blue)
}

fn sum_impossible(input: &str) -> i32 {
    let sum = input
        .split("\n")
        .filter(|s| *s != "")
        .map(Game::new)
        .filter(|game| game.red <= 12 && game.green <= 13 && game.blue <= 14)
        .fold(0, |sum, game| sum + game.id);
    sum
}

#[test]
fn example() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let sum = sum_impossible(input);
    assert_eq!(sum, 8);
}

#[test]
fn parse_game_single_set() {
    let game = Game::new("Game 1: 3 blue, 4 red");
    assert_eq!(
        game,
        Game {
            id: 1,
            blue: 3,
            red: 4,
            green: 0,
        },
    );
}

#[test]
fn parse_game_multiple_sets() {
    let game = Game::new("Game 1: 3 blue, 4 red; 1 green");
    assert_eq!(
        game,
        Game {
            id: 1,
            blue: 3,
            red: 4,
            green: 1,
        },
    );
}
