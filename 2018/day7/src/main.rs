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

/// Find the possible next steps based on the current done and taken steps
fn find_next_step(requirements: &Required, done: &HashSet<Step>, taken: &HashSet<Step>) -> Vec<Step> {
    let mut possible_steps: Vec<Step> = Vec::new();

    for (&step, deps) in requirements {
        if taken.contains(&step) {
            continue;
        }

        if deps.iter().all(|v| done.contains(v)) {
            possible_steps.push(step);
        }
    }

    possible_steps.sort();
    possible_steps.dedup();
    possible_steps.reverse();
    possible_steps
}

fn part1(requirements: &Required) {
    let mut order: Vec<Step> = Vec::new();
    let mut done: HashSet<Step> = HashSet::new();

    loop {
        let mut next_step = find_next_step(&requirements, &done, &done);

        let next_step = match next_step.pop() {
            None => break,
            Some(next_step) => next_step,
        };

        done.insert(next_step);
        order.push(next_step);
    }

    let result: String = order.iter().collect();
    println!("Part1: {}", result);
}

/// Enum that represents the status of a worker
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Status {
    /// That means the worker in on a Idle state.
    Idle,
    /// That contains the step and the remaining seconds
    /// for the worker finish the job.
    Working {
        step: Step,
        remaining: u32,
    },
}

fn apply_tick(workers: &mut Vec<Status>) -> (Vec<Step>) {
    let mut finished_steps: Vec<Step> = Vec::new();

    for index in 0..workers.len() {
        match workers[index] {
            Status::Idle => {}
            Status::Working { step, ref mut remaining } => {
                *remaining -= 1;
                if *remaining == 0 {
                    finished_steps.push(step);
                    workers[index] = Status::Idle;
                }
            }
        }
    }

    finished_steps
}

/// Check if all workers are idle
fn all_workers_idle(workers: &Vec<Status>) -> bool {
    workers.iter().all(|&w| w == Status::Idle)
}

/// Get the index of all available workers
fn available_workers(workers: &Vec<Status>) -> Vec<usize> {
    let mut available: Vec<usize> = Vec::new();

    for index in 0..workers.len() {
        let worker = workers[index];
        if worker == Status::Idle {
            available.push(index);
        }
    }

    available
}

fn compute_step_time(step: &Step) -> u32 {
    (*step as u32) - b'A' as u32 + 1 + 60
}

fn part2(requirements: &Required) {
    // Set that contains the assigned steps
    let mut assigned: HashSet<Step> = HashSet::new();

    // Contains all the registered workers
    let mut workers: Vec<Status> = vec![Status::Idle; 5];

    // Hash set with all finished tasks
    let mut done: HashSet<Step> = HashSet::new();

    let mut seconds: u32 = 0;
    loop {
        let done_steps = apply_tick(&mut workers);
        done.extend(done_steps);
        println!("done => {:?}", done);
        let mut next_tasks = find_next_step(&requirements, &done, &assigned);

        // Check if everything is done in order to break
        if next_tasks.is_empty() && all_workers_idle(&workers) {
            break;
        }

        for index in available_workers(&workers) {
            let step = match next_tasks.pop() {
                None => break,
                Some(step) => step,
            };
            workers[index] = Status::Working {
                step,
                remaining: compute_step_time(&step),
            };
            assigned.insert(step);
        }

        seconds += 1;
    }

    println!("Part2: {}", seconds);
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
    part2(&requirements);
}
