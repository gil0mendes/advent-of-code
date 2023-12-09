#![feature(iter_advance_by)]

use std::{collections::HashMap, fs};

use itertools::Itertools;

use common::*;
use regex::Regex;

fn parse(raw_data: &String) -> Result<Vec<Vec<i32>>> {
    let data = raw_data
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|p| p.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    Ok(data)
}

fn solve(sequence: &Vec<i32>) -> i32 {
    if sequence.iter().all(|&v| v == 0) {
        return 0;
    }

    let mut next_vec = Vec::with_capacity(sequence.len());
    for i in 1..sequence.len() {
        next_vec.push(sequence[i] - sequence[i - 1]);
    }

    sequence.last().unwrap() + solve(&next_vec)
}

fn solve_prev(sequence: &Vec<i32>) -> i32 {
    if sequence.iter().all(|&v| v == 0) {
        return 0;
    }

    let mut next_vec = Vec::with_capacity(sequence.len());
    for i in 1..sequence.len() {
        next_vec.push(sequence[i] - sequence[i - 1]);
    }

    sequence.first().unwrap() - solve_prev(&next_vec)
}

fn compute_part1(raw_data: &String) -> i32 {
    let data = parse(raw_data).unwrap();

    data.iter().map(solve).sum()
}

fn compute_part2(raw_data: &String) -> i32 {
    let data = parse(raw_data).unwrap();

    data.iter().map(solve_prev).sum()
}

fn main() {
    let raw_data = fs::read_to_string("2023/day09/input.txt").expect("input file does not exists");

    println!("Result part 1: {}", compute_part1(&raw_data));
    println!("Result part 2: {}", compute_part2(&raw_data));
}

#[test]
fn test_part1() -> Result<()> {
    const INPUT1: &str = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";
    assert_eq!(compute_part1(&INPUT1.to_string()), 114);

    const INPUT2: &str = "-4 -4 9 49 141 337 751 1623 3426 7039 14036 27210 51589 96428 178991 331412 612714 1129686 2074056 3792092 6922209";
    assert_eq!(compute_part1(&INPUT2.to_string()), 12677569);

    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    const INPUT: &str = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";

    assert_eq!(compute_part2(&INPUT.to_string()), 2);
    Ok(())
}
