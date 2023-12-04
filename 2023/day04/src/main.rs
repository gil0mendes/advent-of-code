use std::{collections::HashSet, fs};

use regex::Regex;

#[derive(Debug)]
struct Card {
    id: u32,
    winner_numbers: HashSet<u32>,
    game_numbers: HashSet<u32>,
}

impl Into<Card> for &str {
    fn into(self) -> Card {
        let re =
            Regex::new(r"Card\s+(?P<card_id>\d+): (?P<winner_numbers>.*) \| (?P<game_numbers>.*)$")
                .unwrap();

        let capture = re.captures_iter(self).next().unwrap();
        let card_id = capture["card_id"].parse::<u32>().unwrap();
        let winner_numbers = (&capture)["winner_numbers"]
            .split(" ")
            .filter_map(|s| s.parse::<u32>().ok())
            .collect::<HashSet<u32>>();
        let game_numbers = (&capture)["game_numbers"]
            .split(" ")
            .filter_map(|s| s.parse::<u32>().ok())
            .collect::<HashSet<u32>>();

        Card {
            id: card_id,
            winner_numbers: winner_numbers,
            game_numbers: game_numbers,
        }
    }
}

fn parse(raw_data: &String) -> Vec<Card> {
    raw_data
        .lines()
        .map(|line| line.into())
        .collect::<Vec<Card>>()
}

fn compute_part1(raw_data: &String) -> u32 {
    let data = parse(raw_data);

    data.iter()
        .map(|card| {
            let correct_count = card.winner_numbers.intersection(&card.game_numbers).count();

            if correct_count == 0 {
                0
            } else {
                2_u32.pow(correct_count as u32 - 1)
            }
        })
        .sum()
}

fn compute_part2(raw_data: &String) -> u32 {
    let data = parse(raw_data);
    let mut num_of_cards = vec![1; data.len()];

    data.iter()
        .map(|card| {
            let correct_count = card.winner_numbers.intersection(&card.game_numbers).count();

            for n in 0..correct_count {
                let index = (card.id + n as u32) as usize;
                num_of_cards[index] += num_of_cards[(card.id - 1) as usize];
            }

            num_of_cards[(card.id - 1) as usize]
        })
        .sum()
}

fn main() {
    let raw_data = fs::read_to_string("2023/day04/input.txt").expect("input file does not exists");

    println!("Result part 1: {}", compute_part1(&raw_data));
    println!("Result part 2: {}", compute_part2(&raw_data));
}

#[test]
fn test_part1() -> Result<(), ()> {
    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    assert_eq!(13, compute_part1(&INPUT.to_string()));
    Ok(())
}

#[test]
fn test_part2() -> Result<(), ()> {
    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    assert_eq!(30, compute_part2(&INPUT.to_string()));
    Ok(())
}
