#![feature(cell_update)]

use std::{cell::Cell, collections::HashSet, fs};

fn compute_part1(raw_data: &String) -> i32 {
    raw_data.lines().filter_map(|v| v.parse::<i32>().ok()).sum()
}

fn compute_part2(raw_data: &String) -> i32 {
    let mut set = HashSet::new();
    let frequency = Cell::new(0);

    raw_data
        .lines()
        .flat_map(|v| v.parse::<i32>().ok())
        .cycle()
        .take_while(|_| set.insert(frequency.get()))
        .for_each(|n| {
            frequency.update(|old| old + n);
        });

    frequency.get()
}

fn main() {
    let raw_data = fs::read_to_string("2018/day1/input.txt").unwrap();

    println!("Result part 1: {}", compute_part1(&raw_data));
    println!("Result part 2: {}", compute_part2(&raw_data))
}

#[test]
fn test_part1() -> Result<(), ()> {
    assert_eq!(compute_part1(&"+1\n+1\n+1".to_string()), 3);
    assert_eq!(compute_part1(&"+1\n+1\n-2".to_string()), 0);
    assert_eq!(compute_part1(&"-1\n-2\n-3".to_string()), -6);
    Ok(())
}

#[test]
fn test_part2() -> Result<(), ()> {
    assert_eq!(compute_part2(&"+1\n-1".to_string()), 0);
    assert_eq!(compute_part2(&"+3\n+3\n+4\n-2\n-4".to_string()), 10);
    assert_eq!(compute_part2(&"-6\n+3\n+8\n+5\n-6".to_string()), 5);
    assert_eq!(compute_part2(&"+7\n+7\n-2\n-7\n-4".to_string()), 14);
    Ok(())
}
