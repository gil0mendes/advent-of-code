use itertools::Itertools;
use regex::Regex;

use std::{fs, str::FromStr};

struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    pub fn is_possible(&self, red: u32, green: u32, blue: u32) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let re = Regex::new(
            r"(Game (?P<game_number>[0-9]+)|(?P<color_count>[0-9]+) (?P<color>green|blue|red))",
        )
        .unwrap();

        let mut captures = re.captures_iter(s);
        let game_number = (captures.next().unwrap()["game_number"])
            .parse::<u32>()
            .unwrap();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for cap in captures {
            red = (&cap["color"] == "red")
                .then(|| cap["color_count"].parse::<u32>().unwrap())
                .map_or(red, |v| v.max(red));

            blue = (&cap["color"] == "blue")
                .then(|| cap["color_count"].parse::<u32>().unwrap())
                .map_or(blue, |v| v.max(blue));

            green = (&cap["color"] == "green")
                .then(|| cap["color_count"].parse::<u32>().unwrap())
                .map_or(green, |v| v.max(green));
        }

        Ok(Self {
            id: game_number,
            red,
            green,
            blue,
        })
    }
}

fn compute_part1(raw_data: &String) -> u32 {
    raw_data
        .lines()
        .map(|line| Game::from_str(line).unwrap())
        .filter_map(|game| game.is_possible(12, 13, 14).then(|| game.id))
        .dedup_by(|a, b| a == b)
        .sum()
}

fn compute_part2(raw_data: &String) -> u32 {
    raw_data
        .lines()
        .map(|line| Game::from_str(line).unwrap())
        .map(|game| game.red * game.green * game.blue)
        .sum()
}

fn main() {
    let raw_data = fs::read_to_string("2023/day02/input.txt").expect("input file does not exists");

    println!("Result part 1: {}", compute_part1(&raw_data));
    println!("Result part 2: {}", compute_part2(&raw_data));
}

#[test]
fn test_part1() -> Result<(), ()> {
    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(8, compute_part1(&INPUT.to_string()));
    Ok(())
}

#[test]
fn test_part2() -> Result<(), ()> {
    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(2286, compute_part2(&INPUT.to_string()));
    Ok(())
}
