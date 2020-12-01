use std::iter::FromIterator;
use std::{collections::HashSet, fs};

const TARGET: u32 = 2020;

/// makes use of a HashSet to get an entry that possibly matches the criteria. When found it multiplies to get the
/// result.
fn find_two(expenses: &Vec<u32>) -> Option<u32> {
    let entries = HashSet::<u32>::from_iter(expenses.iter().copied());

    entries
        .iter()
        .find_map(|x| entries.get(&(TARGET - x)).map(|x| x * x))
}

/// uses the same approach as the first, but with an additional variable.
fn find_three(expenses: &Vec<u32>) -> Option<u32> {
    let entries = HashSet::<u32>::from_iter(expenses.iter().copied());

    entries.iter().find_map(|x| {
        entries
            .iter()
            .filter(|&y| y != x)
            .find_map(|y| entries.get(&(TARGET - x - y)).map(|z| x * y * z))
    })
}

fn main() {
    let file_content = fs::read_to_string("input.txt").expect("File doesn't exists");

    let all_expenses: Vec<u32> = file_content
        .trim()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Part 1: {}", find_two(&all_expenses).unwrap());
    println!("Part 2: {}", find_three(&all_expenses).unwrap());
}
