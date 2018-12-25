#![feature(drain_filter)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::error::Error;

use regex::Regex;

const DATA: &str = include_str!("./input.txt");

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

/// Just to make the code more semantic.
type Step = char;

/// Type that represents the requirements for each step.
type Required = HashMap<Step, HashSet<Step>>;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Dependency {
    step: Step,
    required: Step,
}

impl Dependency {}

/// Implement parsers for a Dependency
impl FromStr for Dependency {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static! {
            static ref REG: Regex = Regex::new(
                r"Step (?P<dependency>[A-Z]) must be finished before step (?P<step>[A-Z]) can begin."
            ).unwrap();
        }

        let caps = match REG.captures(s) {
            None => return err!("Unrecognized dependency"),
            Some(caps) => caps,
        };

        Ok(Dependency {
            step: caps["step"].chars().next().unwrap(),
            required: caps["dependency"].chars().next().unwrap(),
        })
    }
}

fn find_next_step(requirements: &Required, done: &HashSet<Step>) -> Option<Step> {
    let mut possible_steps: Vec<Step> = Vec::new();

    for (&step, deps) in requirements {
        if done.contains(&step) {
            continue;
        }

        if deps.iter().all(|v| done.contains(v)) {
            possible_steps.push(step);
        }
    }

    possible_steps.sort();
    possible_steps.dedup();
    possible_steps.reverse();
    possible_steps.pop()
}

fn part1(requirements: &Required) {
    let mut order: Vec<Step> = Vec::new();
    let mut done: HashSet<Step> = HashSet::new();

    loop {
        let next_step = find_next_step(&requirements, &done);

        let next_step = match next_step {
            None => break,
            Some(next_step) => next_step,
        };

        done.insert(next_step);
        order.push(next_step);
    }

    let result: String = order.iter().collect();
    println!("Part1: {}", result);
}

fn main() {
    // Build the list of dependencies
    let dependencies = DATA
        .lines()
        .map(|line| line.parse::<Dependency>())
        .collect::<Result<Vec<Dependency>, _>>()
        .unwrap();

    // Build map with all dependencies for each step.
    let mut requirements: Required = HashMap::new();
    dependencies
        .iter()
        .for_each(|dep| {
            requirements.entry(dep.step).or_default().insert(dep.required);
            requirements.entry(dep.required).or_default();
        });

    part1(&requirements);
}
