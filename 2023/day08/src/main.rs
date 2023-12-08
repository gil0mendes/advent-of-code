#![feature(iter_advance_by)]

use std::{collections::HashMap, fs};

use itertools::Itertools;

use common::*;
use regex::Regex;

fn parse(raw_data: &String) -> Result<(Vec<char>, HashMap<&str, (&str, &str)>)> {
    let mut lines = raw_data.lines();

    let turns = lines.next().unwrap().chars().collect_vec();
    lines.next();

    let mut directions = HashMap::<&str, (&str, &str)>::new();
    let rg = Regex::new(r"(\w+)").unwrap();
    for line in lines {
        let captures = rg.find_iter(line).map(|e| e.as_str()).collect_vec();

        directions.insert(captures[0], (captures[1], captures[2]));
    }

    Ok((turns, directions))
}

fn compute_part1(raw_data: &String) -> u32 {
    let (turns, directions) = parse(raw_data).unwrap();

    let number_of_turns = turns.len();

    let mut current_location = "AAA";
    let mut num_steps = 0;
    loop {
        let turn_to_take = turns[num_steps % number_of_turns];
        let location = directions.get(current_location).expect("invalid location");

        current_location = match turn_to_take {
            'L' => location.0,
            'R' => location.1,
            _ => panic!("invalid turn"),
        };
        num_steps += 1;

        if current_location == "ZZZ" {
            break;
        }
    }

    num_steps as u32
}

fn compute_part2(raw_data: &String) -> usize {
    let (turns, directions) = parse(raw_data).unwrap();

    let starts = directions
        .keys()
        .filter(|&key| key.chars().last().unwrap() == 'A')
        .map(|k| *k)
        .collect_vec();

    starts
        .into_iter()
        .map(|mut node| {
            turns
                .iter()
                .cycle()
                .positions(|&turn| {
                    let location = directions.get(node).unwrap();

                    node = match turn {
                        'L' => location.0,
                        'R' => location.1,
                        _ => panic!("invalid turn"),
                    };

                    node.chars().last().unwrap() == 'Z'
                })
                .map(|pos| pos + 1)
                .next()
                .expect("infinity loop")
        })
        .reduce(num_integer::lcm)
        .unwrap()
}

fn main() {
    let raw_data = fs::read_to_string("2023/day08/input.txt").expect("input file does not exists");

    println!("Result part 1: {}", compute_part1(&raw_data));
    println!("Result part 2: {}", compute_part2(&raw_data));
}

#[test]
fn test_part1() -> Result<()> {
    const INPUT1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    assert_eq!(compute_part1(&INPUT1.to_string()), 2);
    assert_eq!(compute_part1(&INPUT2.to_string()), 6);
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    const INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    assert_eq!(compute_part2(&INPUT.to_string()), 6);
    Ok(())
}
