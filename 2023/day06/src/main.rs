#![feature(iter_advance_by)]

use std::{fs, ops::Div, str::Lines, vec};

use common::parse_line_numbers;
use itertools::Itertools;

// distanceTraveled = (raceTime - buttonPressTime) * buttonPressTime
// buttonPress = raceTime/2 +- sqrt(raceTime^2 - 4*distance)/2

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub fn new(time: u64, distance: u64) -> Self {
        Self { distance, time }
    }
}

fn parse(raw_data: &String) -> Vec<Race> {
    let mut lines = raw_data.lines();

    let times = parse_line_numbers(lines.next().unwrap());
    let distances = parse_line_numbers(lines.next().unwrap());

    times
        .iter()
        .enumerate()
        .map(|(idx, &time)| Race::new(time, distances[idx]))
        .collect_vec()
}

fn solver(race: &Race) -> u64 {
    let half_time = race.time as f64 / 2_f64;
    let sqrt_result = ((race.time.pow(2) - 4 * (race.distance + 1)) as f64).sqrt() / 2_f64;
    let lower_bound = (half_time - sqrt_result).ceil();
    let upper_bound = (half_time + sqrt_result).floor().min(race.time as f64);

    (upper_bound - lower_bound) as u64 + 1
}

fn compute_part1(raw_data: &String) -> u64 {
    let data = parse(raw_data);

    data.iter().map(solver).product()
}

fn compute_part2(raw_data: &String) -> u64 {
    let mut lines = raw_data.lines();

    let time = lines
        .next()
        .map(|line| {
            line.split(':')
                .last()
                .unwrap()
                .replace(' ', "")
                .parse::<u64>()
                .unwrap()
        })
        .unwrap();

    let distance = lines
        .next()
        .map(|line| {
            line.split(':')
                .last()
                .unwrap()
                .replace(' ', "")
                .parse::<u64>()
                .unwrap()
        })
        .unwrap();

    solver(&Race { time, distance })
}

fn main() {
    let raw_data = fs::read_to_string("2023/day06/input.txt").expect("input file does not exists");

    println!("Result part 1: {}", compute_part1(&raw_data));
    println!("Result part 2: {}", compute_part2(&raw_data));
}

#[test]
fn test_part1() -> Result<(), ()> {
    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    assert_eq!(compute_part1(&INPUT.to_string()), 288);
    Ok(())
}

#[test]
fn test_part2() -> Result<(), ()> {
    const INPUT: &str = "Time:      7  15   30
    Distance:  9  40  200";

    assert_eq!(compute_part2(&INPUT.to_string()), 71503);
    Ok(())
}
